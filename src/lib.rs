// Encodes and decodes Erlang external form format.

mod constants;

use std::{io, str};
use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};
use encoding_rs::WINDOWS_1252;
use thiserror::Error;
use num::bigint::BigInt;
use num::bigint::Sign;

//
// Types
//

pub type DecodingResult = Result<ErlangExtTerm, DecodingError>;

#[derive(Error, Debug)]
pub enum DecodingError {
    #[error("unrecognized external term format tag")]
    UnrecognizedTag {
        tag: u8
    },
    #[error("unrecognized data type marker")]
    UnrecognizedType {
        value: u8
    },
    #[error("failed to decode payload into a UTF-8 string")]
    DecodingFailure(#[from] io::Error),
    #[error("format version is unsupported")]
    UnsupportedVersion {
        version: u8
    },
    #[error("other types of errors")]
    Other,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErlangExtTerm {
    Atom(String),
    SmallInteger(u8),
    Integer(i32),
    BigInteger(BigInt),
    Float(f64),
    BitBinary(Vec<u8>, u8),
    Binary(Vec<u8>)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Atom {
    pub name: String
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BigInteger {
    pub value: BigInt
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BitBinary {
    pub bytes: Vec<u8>,
    pub num_of_trailing_bits: u8
}

//
// Decoding
//

impl ErlangExtTerm {
    pub fn decode(reader: Box<dyn io::Read>) -> DecodingResult {
        return Decoder::new(reader).decode();
    }
}

pub struct Decoder {
    reader: Box<dyn io::Read>,
    buffer: Vec<u8>
}

impl Decoder {
    pub fn new(reader: Box<dyn io::Read>) -> Self {
        Decoder {
            reader,
            buffer: Vec::new()
        }
    }

    pub fn decode(mut self) -> DecodingResult {
        let version = self.reader.read_u8()?;
        if version != constants::TERM_FORMAT_VERSION {
            return Err(DecodingError::UnsupportedVersion { version });
        }

        let tag = self.reader.read_u8()?;
        // TODO: distribution header
        // TODO: compressed term
        return self.decode_tagged_with(tag);
    }

    fn decode_tagged_with(&mut self, tag: u8) -> DecodingResult {
        match tag {
            constants::ATOM_EXT => self.decode_atom_ext(),
            constants::ATOM_UTF8_EXT => self.decode_atom_utf8_ext(),
            constants::SMALL_ATOM_UTF8_EXT => self.decode_small_atom_utf8_ext(),
            constants::SMALL_INTEGER_EXT => self.decode_small_integer(),
            constants::INTEGER_EXT => self.decode_integer(),
            constants::SMALL_BIG_EXT => self.decode_small_big_integer(),
            constants::LARGE_BIG_EXT => self.decode_large_big_integer(),
            constants::NEW_FLOAT_EXT => self.decode_float(),
            constants::BINARY_EXT => self.decode_binary(),
            constants::BIT_BINARY_EXT => self.decode_bit_binary(),
            _ => Err(DecodingError::UnrecognizedTag { tag }),
        }
    }

    // Legacy atom encoding format, assumes Latin1 (Windows-1252) encoding
    fn decode_atom_ext(&mut self) -> DecodingResult {
        let length = self.reader.read_u16::<BigEndian>()?;
        self.buffer.resize(length as usize, 0);
        self.reader.read_exact(&mut self.buffer)?;

        let (s, _, had_errors) = WINDOWS_1252.decode(&self.buffer);
        if had_errors {
            let e = io::Error::new(io::ErrorKind::InvalidData, s.to_string());
            return Err(DecodingError::DecodingFailure(e));
        } else {
            return Ok(ErlangExtTerm::Atom(s.to_string()));
        }
    }

    // Modern atom encoding format, assumes UTF-8 encoding
    fn decode_atom_utf8_ext(&mut self) -> DecodingResult {
        let length = self.reader.read_u16::<BigEndian>()?;
        self.buffer.resize(length as usize, 0);
        self.reader.read_exact(&mut self.buffer)?;

        match str::from_utf8(&self.buffer) {
            Ok(s)  => Ok(ErlangExtTerm::Atom(s.to_string())),
            Err(e) => {
                let io_e = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
                Err(DecodingError::DecodingFailure(io_e))
            }
        }
    }

    // Modern atom encoding format, assumes UTF-8 encoding
    fn decode_small_atom_utf8_ext(&mut self) -> DecodingResult {
        let length: u8 = self.reader.read_u8()?;
        self.buffer.resize(length as usize, 0);
        self.reader.read_exact(&mut self.buffer)?;

        match str::from_utf8(&self.buffer) {
            Ok(s)  => Ok(ErlangExtTerm::Atom(s.to_string())),
            Err(e) => {
                let io_e = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
                Err(DecodingError::DecodingFailure(io_e))
            }
        }
    }

    fn decode_small_integer(&mut self) -> DecodingResult {
        match self.reader.read_u8() {
            Ok(i)  => Ok(ErlangExtTerm::SmallInteger(i)),
            Err(e) => {
                let io_e = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
                Err(DecodingError::DecodingFailure(io_e))
            }
        }
    }

    fn decode_integer(&mut self) -> DecodingResult {
        match self.reader.read_i32::<BigEndian>() {
            Ok(i)  => Ok(ErlangExtTerm::Integer(i)),
            Err(e) => {
                let io_e = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
                Err(DecodingError::DecodingFailure(io_e))
            }
        }
    }

    fn decode_small_big_integer(&mut self) -> DecodingResult {
        let n = self.reader.read_u8()? as usize;
        let sign = self.reader.read_u8()?;

        self.buffer.resize(n, 0);
        self.reader.read_exact(&mut self.buffer)?;

        // section 12.18:
        // The digits are stored with the least significant byte stored first.
        let val = BigInt::from_bytes_le(to_sign(sign)?, &self.buffer);
        Ok(ErlangExtTerm::BigInteger(val))
    }

    fn decode_large_big_integer(&mut self) -> DecodingResult {
        let n = self.reader.read_u32::<BigEndian>()? as usize;
        let sign = self.reader.read_u8()?;

        self.buffer.resize(n, 0);
        self.reader.read_exact(&mut self.buffer)?;

        // section 12.18:
        // The digits are stored with the least significant byte stored first.
        let val = BigInt::from_bytes_le(to_sign(sign)?, &self.buffer);
        Ok(ErlangExtTerm::BigInteger(val))
    }

    fn decode_float(&mut self) -> DecodingResult {
        match self.reader.read_f64::<BigEndian>() {
            Ok(i)  => Ok(ErlangExtTerm::Float(i)),
            Err(e) => {
                let io_e = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
                Err(DecodingError::DecodingFailure(io_e))
            }
        }
    }

    fn decode_binary(&mut self) -> DecodingResult {
        let n = self.reader.read_u32::<BigEndian>()? as usize;
        let mut input = vec![0; n];

        self.reader.read_exact(&mut input)?;
        Ok(ErlangExtTerm::Binary(input))
    }

    fn decode_bit_binary(&mut self) -> DecodingResult {
        let n = self.reader.read_u32::<BigEndian>()? as usize;
        let tail_len = self.reader.read_u8()?;

        let mut input = vec![0; n];
        self.reader.read_exact(&mut input)?;
        if !input.is_empty() {
            let shift_by = 8 - tail_len;
            let tail = input[n - 1] >> shift_by;
            input[n - 1] = tail;
        }
        Ok(ErlangExtTerm::BitBinary(input, tail_len))
    }
}

pub fn to_sign(i: u8) -> io::Result<Sign> {
    match i {
        0 => Ok(Sign::Plus),
        1 => Ok(Sign::Minus),
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, format!("BigInteger sign must be either 0 or 1, given: {}", i)))
    }
}

impl From<Atom> for ErlangExtTerm {
    fn from(val: Atom) -> Self {
        ErlangExtTerm::Atom(val.name)
    }
}
impl From<String> for ErlangExtTerm {
    fn from(val: String) -> Self {
        ErlangExtTerm::Atom(val)
    }
}
impl From<&str> for ErlangExtTerm {
    fn from(val: &str) -> Self {
        ErlangExtTerm::Atom(val.to_string())
    }
}
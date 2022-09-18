// Encodes and decodes Erlang external form format.

mod constants;

use std::{io, str};
use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};
use encoding_rs::WINDOWS_1252;
use thiserror::Error;

//
// Types
//

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ErlangExtTerm {
    Atom(String)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Atom {
    pub name: String
}

//
// Decoding
//

impl ErlangExtTerm {
    pub fn decode(reader: Box<dyn io::Read>) -> Result<ErlangExtTerm, DecodingError> {
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

    pub fn decode(mut self) -> Result<ErlangExtTerm, DecodingError> {
        let version = self.reader.read_u8()?;
        if version != constants::TERM_FORMAT_VERSION {
            return Err(DecodingError::UnsupportedVersion { version });
        }

        let tag = self.reader.read_u8()?;
        // TODO: distribution header
        // TODO: compressed term
        return self.decode_tagged_with(tag);
    }

    fn decode_tagged_with(&mut self, tag: u8) -> Result<ErlangExtTerm, DecodingError> {
        match tag {
            constants::ATOM_EXT => self.decode_atom_ext(),
            constants::ATOM_UTF8_EXT => self.decode_atom_utf8_ext(),
            constants::SMALL_ATOM_UTF8_EXT => self.decode_small_atom_utf8_ext(),
            _ => Err(DecodingError::UnrecognizedTag { tag }),
        }
    }

    // Legacy atom encoding format, assumes Latin1 (Windows-1252) encoding
    fn decode_atom_ext(&mut self) -> Result<ErlangExtTerm, DecodingError> {
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
    fn decode_atom_utf8_ext(&mut self) -> Result<ErlangExtTerm, DecodingError> {
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
    fn decode_small_atom_utf8_ext(&mut self) -> Result<ErlangExtTerm, DecodingError> {
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
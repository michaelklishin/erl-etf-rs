use byteorder::{BigEndian, ReadBytesExt};
use encoding_rs::WINDOWS_1252;
use std::io::Read;
use std::{io, str};

use crate::*;
use crate::constants;
use crate::numerical::*;

pub struct Decoder {
    reader: Box<dyn io::Read>,
    buffer: Vec<u8>,
}

impl Decoder {
    pub fn new(reader: Box<dyn io::Read>) -> Self {
        Decoder {
            reader,
            buffer: Vec::new(),
        }
    }

    pub fn decode(&mut self) -> DecodingResult {
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
            constants::NEW_PID_EXT => self.decode_pid(),
            constants::NEW_PORT_EXT => self.decode_v3_port(),
            constants::V4_PORT_EXT => self.decode_v4_port(),
            constants::SMALL_TUPLE_EXT => self.decode_small_tuple(),
            constants::LARGE_TUPLE_EXT => self.decode_large_tuple(),
            constants::NIL_EXT => self.decode_nil(),
            constants::LIST_EXT => self.decode_list(),
            constants::NEWER_REFERENCE_EXT => self.decode_newer_reference(),
            constants::FUN_EXPORT_EXT => self.decode_external_fun(),
            constants::NEW_FUN_EXT => self.decode_internal_fun(),
            _ => Err(DecodingError::UnrecognizedTag { tag }),
        }
    }

    fn read_next_term(&mut self) -> DecodingResult {
        let term_tag = self.reader.read_u8()?;
        return self.decode_tagged_with(term_tag);
    }

    fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        return self.reader.read_u8();
    }

    fn read_u16(&mut self) -> Result<u16, std::io::Error> {
        return self.reader.read_u16::<BigEndian>();
    }

    fn read_u32(&mut self) -> Result<u32, std::io::Error> {
        return self.reader.read_u32::<BigEndian>();
    }

    fn read_i32(&mut self) -> Result<i32, std::io::Error> {
        return self.reader.read_i32::<BigEndian>();
    }

    fn read_u64(&mut self) -> Result<u64, std::io::Error> {
        return self.reader.read_u64::<BigEndian>();
    }

    fn read_f64(&mut self) -> Result<f64, std::io::Error> {
        return self.reader.read_f64::<BigEndian>();
    }

    // Legacy atom encoding format, assumes Latin1 (Windows-1252) encoding
    fn decode_atom_ext(&mut self) -> DecodingResult {
        let length = self.read_u16()? as usize;
        self.buffer.resize(length, 0);
        self.reader.read_exact(&mut self.buffer)?;

        let (s, _, had_errors) = WINDOWS_1252.decode(&self.buffer);
        if had_errors {
            let e = io::Error::new(io::ErrorKind::InvalidData, s.to_string());
            return Err(DecodingError::DecodingFailure(e));
        } else {
            return Ok(ErlTerm::Atom(s.to_string()));
        }
    }

    // Modern atom encoding format, assumes UTF-8 encoding
    fn decode_atom_utf8_ext(&mut self) -> DecodingResult {
        let length = self.read_u16()? as usize;
        self.buffer.resize(length, 0);
        self.reader.read_exact(&mut self.buffer)?;

        match str::from_utf8(&self.buffer) {
            Ok(s) => Ok(ErlTerm::Atom(s.to_string())),
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
            Ok(s) => Ok(ErlTerm::Atom(s.to_string())),
            Err(e) => {
                let io_e = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
                Err(DecodingError::DecodingFailure(io_e))
            }
        }
    }

    fn decode_small_integer(&mut self) -> DecodingResult {
        match self.reader.read_u8() {
            Ok(i) => Ok(ErlTerm::SmallInteger(i)),
            Err(e) => {
                let io_e = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
                Err(DecodingError::DecodingFailure(io_e))
            }
        }
    }

    fn decode_integer(&mut self) -> DecodingResult {
        match self.read_i32() {
            Ok(i) => Ok(ErlTerm::Integer(i)),
            Err(e) => {
                let io_e = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
                Err(DecodingError::DecodingFailure(io_e))
            }
        }
    }

    fn decode_small_big_integer(&mut self) -> DecodingResult {
        let n = self.read_u8()? as usize;
        let sign = self.reader.read_u8()?;

        self.buffer.resize(n, 0);
        self.reader.read_exact(&mut self.buffer)?;

        // section 12.18:
        // The digits are stored with the least significant byte stored first.
        let val = BigInt::from_bytes_le(to_sign(sign)?, &self.buffer);
        Ok(ErlTerm::BigInteger(val))
    }

    fn decode_large_big_integer(&mut self) -> DecodingResult {
        let n = self.read_u32()? as usize;
        let sign = self.reader.read_u8()?;

        self.buffer.resize(n, 0);
        self.reader.read_exact(&mut self.buffer)?;

        // section 12.18:
        // The digits are stored with the least significant byte stored first.
        let val = BigInt::from_bytes_le(to_sign(sign)?, &self.buffer);
        Ok(ErlTerm::BigInteger(val))
    }

    fn decode_float(&mut self) -> DecodingResult {
        match self.read_f64() {
            Ok(i) => Ok(ErlTerm::Float(OrderedFloat::<f64>(i as f64))),
            Err(e) => {
                let io_e = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
                Err(DecodingError::DecodingFailure(io_e))
            }
        }
    }

    fn decode_binary(&mut self) -> DecodingResult {
        let n = self.read_u32()? as usize;
        let mut input = vec![0; n];

        self.reader.read_exact(&mut input)?;
        Ok(ErlTerm::Binary(input))
    }

    fn decode_bit_binary(&mut self) -> DecodingResult {
        let n = self.read_u32()? as usize;
        let tail_len = self.reader.read_u8()?;

        let mut input = vec![0; n];
        self.reader.read_exact(&mut input)?;
        if !input.is_empty() {
            let shift_by = 8 - tail_len;
            let tail = input[n - 1] >> shift_by;
            input[n - 1] = tail;
        }
        Ok(ErlTerm::BitBinary(input, tail_len))
    }

    fn decode_pid(&mut self) -> DecodingResult {
        let node = self.read_next_term()?;
        match TryInto::<Atom>::try_into(node) {
            Ok(val) => {
                let id = self.read_u32()?;
                let serial = self.read_u32()?;
                let creation = self.read_u32()?;

                Ok(ErlTerm::Pid(ErlPid {
                    node: val,
                    id,
                    serial,
                    creation,
                }))
            }
            _ => Err(DecodingError::CompoundTypeDecodingFailure()),
        }
    }

    fn decode_v3_port(&mut self) -> DecodingResult {
        let node = self.read_next_term()?;
        match TryInto::<Atom>::try_into(node) {
            Ok(val) => {
                let id = self.read_u32()?;
                let creation = self.read_u32()?;

                Ok(ErlTerm::V3Port(ErlV3Port {
                    node: val,
                    id,
                    creation,
                }))
            }
            _ => Err(DecodingError::CompoundTypeDecodingFailure()),
        }
    }

    fn decode_v4_port(&mut self) -> DecodingResult {
        let node = self.read_next_term()?;
        match TryInto::<Atom>::try_into(node) {
            Ok(val) => {
                let id = self.read_u64()?;
                let creation = self.read_u32()?;

                Ok(ErlTerm::V4Port(ErlV4Port {
                    node: val,
                    id,
                    creation,
                }))
            }
            _ => Err(DecodingError::CompoundTypeDecodingFailure()),
        }
    }

    fn decode_small_tuple(&mut self) -> DecodingResult {
        let n = self.read_u8()? as usize;
        let mut items = Vec::with_capacity(n);

        for _i in 0..n {
            match self.read_next_term() {
                Ok(term) => items.push(term),
                Err(_) => return Err(DecodingError::CompoundTypeDecodingFailure()),
            }
        }

        Ok(ErlTerm::Tuple(Tuple { elements: items }))
    }

    fn decode_large_tuple(&mut self) -> DecodingResult {
        let n = self.read_u32()? as usize;
        let mut items = Vec::with_capacity(n);

        for _i in 0..n {
            match self.read_next_term() {
                Ok(term) => items.push(term),
                Err(_) => return Err(DecodingError::CompoundTypeDecodingFailure())
            }
        }

        Ok(ErlTerm::Tuple(Tuple { elements: items }))
    }

    fn decode_list(&mut self) -> DecodingResult {
        let n = self.read_u32()? as usize;
        let mut items = Vec::with_capacity(n);

        for _i in 0..n {
            match self.read_next_term() {
                Ok(term) => items.push(term),
                Err(_) => return Err(DecodingError::CompoundTypeDecodingFailure())
            }
        }
        let tail_term = self.read_next_term()?;
        match tail_term {
            ErlTerm::List(val) =>
                if val.is_nil() {
                    return Ok(ErlTerm::List(List { elements: items }))
                } else {
                    // this is an improper list
                    return Ok(ErlTerm::ImproperList(ImproperList {
                        elements: items, tail: Box::new(ErlTerm::List(val))
                    }))
                },
            other =>
                // this is an improper list
                return Ok(ErlTerm::ImproperList(ImproperList {
                    elements: items, tail: Box::new(other)
                }))
        }
    }

    fn decode_nil(&mut self) -> DecodingResult {
        return Ok(ErlTerm::List(List::nil()));
    }

    fn decode_newer_reference(&mut self) -> DecodingResult {
        let arity = self.read_u16()? as usize;
        let node = self.read_next_term()?;
        match TryInto::<Atom>::try_into(node) {
            Ok(atom) => {
                let creation = self.read_u32()?;
                // remaining ref ID bytes
                let mut tail = Vec::<u32>::with_capacity(arity);

                for _i in 0..arity {
                    let j = self.read_u32()?;
                    tail.push(j);
                }

                return Ok(ErlTerm::Ref(Ref { node: atom, creation, id: tail }));
            },
            Err(_) => return Err(DecodingError::CompoundTypeDecodingFailure())

        }
    }

    fn decode_external_fun(&mut self) -> DecodingResult {
        let module_term = self.read_next_term()?;
        let module_atom = TryInto::<Atom>::try_into(module_term).unwrap();

        let fn_name_term = self.read_next_term()?;
        let fn_name_atom = TryInto::<Atom>::try_into(fn_name_term).unwrap();

        let arity_term = self.read_next_term()?;
        let arity = TryInto::<u8>::try_into(arity_term).unwrap();

        Ok(ErlTerm::ExternalFun(
            ExternalFun {
                module: module_atom,
                function_name: fn_name_atom,
                arity
            }
        ))
    }

    fn decode_internal_fun(&mut self) -> DecodingResult {
        let _size = self.read_u32()?;
        let arity = self.read_u8()?;

        let mut uniq_beam_md5 = [0; 16];
        let _ = self.reader.read_exact(&mut uniq_beam_md5);

        let idx = self.read_u32()?;
        let free_variable_count = self.read_u32()?;

        let module_atom: Atom = self.read_next_term()?.try_into().unwrap();

        let old_idx: i32 = self.read_next_term()?.try_into().unwrap();
        let old_uniq: i32 = self.read_next_term()?.try_into().unwrap();
        let creator_pid: ErlPid = self.read_next_term()?.try_into().unwrap();
        let mut free_vars = Vec::with_capacity(free_variable_count as usize);
        for _i in 0..free_variable_count {
            free_vars.push(self.read_next_term()?);
        }

        Ok(ErlTerm::InternalFun(InternalFun {
            arity,
            free_variable_count,
            uniq_beam_md5,
            index: idx,
            module: module_atom,
            old_index: old_idx,
            old_uniq_hash: old_uniq,
            creator_pid,
            free_vars
        }))
    }
}

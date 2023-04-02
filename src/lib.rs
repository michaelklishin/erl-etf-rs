// Encodes and decodes Erlang external form format.

mod constants;
mod decoding;
mod conversions;
mod numerical;

use std::io;

use num::bigint::BigInt;
use ordered_float::OrderedFloat;
use std::convert::TryInto;
use thiserror::Error;

use decoding::Decoder;

//
// Types
//

pub type DecodingResult = Result<ErlTerm, DecodingError>;

#[derive(Error, Debug)]
pub enum DecodingError {
    #[error("unrecognized external term format tag")]
    UnrecognizedTag { tag: u8 },
    #[error("unrecognized data type marker")]
    UnrecognizedType { value: u8 },
    #[error("failed to decode payload into a UTF-8 string")]
    DecodingFailure(#[from] io::Error),
    #[error("failed to decode a compound term data type")]
    CompoundTypeDecodingFailure(),
    #[error("format version is unsupported")]
    UnsupportedVersion { version: u8 },
    #[error("other types of errors")]
    Other,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ErlTerm {
    Atom(String),
    SmallInteger(u8),
    Integer(i32),
    BigInteger(BigInt),
    Float(OrderedFloat<f64>),
    BitBinary(Vec<u8>, u8),
    Binary(Vec<u8>),
    Pid(ErlPid),
    V3Port(ErlV3Port),
    V4Port(ErlV4Port),
    Tuple(Tuple),
    List(List),
    ImproperList(ImproperList),
    Ref(Ref),
    ExternalFun(ExternalFun),
    InternalFun(InternalFun)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Atom {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BigInteger {
    pub value: BigInt,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BitBinary {
    pub bytes: Vec<u8>,
    pub num_of_trailing_bits: u8,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ErlPid {
    pub node: Atom,
    pub id: u32,
    pub serial: u32,
    pub creation: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ErlV3Port {
    pub node: Atom,
    pub id: u32,
    pub creation: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ErlV4Port {
    pub node: Atom,
    pub id: u64,
    pub creation: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tuple {
    pub elements: Vec<ErlTerm>,
}
impl Tuple {
    pub fn empty() -> Self {
        Tuple {
            elements: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct List {
    pub elements: Vec<ErlTerm>,
}
impl List {
    pub fn nil() -> Self {
        return Self::empty();
    }

    pub fn is_nil(&self) -> bool {
        return self.elements.is_empty();
    }

    pub fn empty() -> Self {
        List {
            elements: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ImproperList {
    pub elements: Vec<ErlTerm>,
    pub tail: Box<ErlTerm>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Ref {
    pub node: Atom,
    pub creation: u32,
    // an opaque sequence of bytes
    pub id: Vec<u32>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ExternalFun {
    pub module: Atom,
    pub function_name: Atom,
    pub arity: u8
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct InternalFun {
    pub arity: u8,
    pub uniq_beam_md5: [u8; 16],
    pub index: u32,
    pub free_variable_count: u32,
    pub module: Atom,
    pub old_index: i32,
    pub old_uniq_hash: i32,
    pub creator_pid: ErlPid,
    pub free_vars: Vec<ErlTerm>
}

//
// Decoding
//

impl ErlTerm {
    pub fn decode(reader: Box<dyn io::Read>) -> DecodingResult {
        return Decoder::new(reader).decode();
    }
}


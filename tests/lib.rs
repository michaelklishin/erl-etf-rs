extern crate erl_etf;

use erl_etf::*;
use num::bigint::ToBigInt;
use ordered_float::OrderedFloat;
use std::io::Cursor;

//
// Decoding
//

#[test]
fn decode_atom() {
    // 8> term_to_binary(a).
    // <<131,100,0,1,97>>
    let res1 = ErlangExtTerm::decode(binary_data(&[131, 100, 0, 1, 97])).unwrap();
    assert_eq!(atom("a"), res1);
    // 9> term_to_binary(b).
    // <<131,100,0,1,98>>
    let res2 = ErlangExtTerm::decode(binary_data(&[131, 100, 0, 1, 98])).unwrap();
    assert_eq!(atom("b"), res2);
    // 11> term_to_binary(erlang).
    // <<131,100,0,6,101,114,108,97,110,103>>
    let input3 = binary_data(&[131, 100, 0, 6, 101, 114, 108, 97, 110, 103]);
    assert_eq!(atom("erlang"), ErlangExtTerm::decode(input3).unwrap());
    // 12> term_to_binary(rust).
    // <<131,100,0,4,114,117,115,116>>
    let input4 = binary_data(&[131, 100, 0, 4, 114, 117, 115, 116]);
    let res4 = ErlangExtTerm::decode(input4).unwrap();
    assert_eq!(atom("rust"), res4);
    // 10> term_to_binary('Cádiz').
    // <<131,100,0,5,67,225,100,105,122>>
    let input5 = binary_data(&[131, 100, 0, 5, 67, 225, 100, 105, 122]);
    let res5 = ErlangExtTerm::decode(input5).unwrap();
    assert_eq!(atom("Cádiz"), res5);
    // 12> term_to_binary('Эрланг').
    // <<131,119,12,208,173,209,128,208,187,208,176,208,189,208,179>>
    // uses SMALL_ATOM_UTF8_EXT
    let input6 = binary_data(&[
        131, 119, 12, 208, 173, 209, 128, 208, 187, 208, 176, 208, 189, 208, 179,
    ]);
    let res6 = ErlangExtTerm::decode(input6).unwrap();
    assert_eq!(atom("Эрланг"), res6);
    // uses ATOM_UTF8_EXT
    let input7 = binary_data(&[
        131, 118, 1, 226, 208, 174, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142, 209, 142,
        209, 142, 209, 142, 209, 142, 209, 142, 208, 189, 208, 184, 208, 186, 208, 190, 208, 180,
    ]);
    let res7 = ErlangExtTerm::decode(input7).unwrap();
    let s = "Ююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююююникод";
    assert_eq!(atom(s), res7);
}

#[test]
fn decode_small_integer() {
    // 1> term_to_binary(1).
    // <<131,97,1>>
    let input1 = binary_data(&[131, 97, 1]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(small_integer(1), res1);
    // 11> term_to_binary(255).
    // <<131,97,255>>
    let input2 = binary_data(&[131, 97, 255]);
    let res2 = ErlangExtTerm::decode(input2).unwrap();
    assert_eq!(small_integer(255), res2);
}

#[test]
fn decode_integer() {
    // 10> term_to_binary(256).
    // <<131,98,0,0,1,0>>
    let input1 = binary_data(&[131, 98, 0, 0, 1, 0]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(integer(256), res1);
    // 3> term_to_binary(1000).
    // <<131,98,0,0,3,232>>
    let input2 = binary_data(&[131, 98, 0, 0, 3, 232]);
    let res2 = ErlangExtTerm::decode(input2).unwrap();
    assert_eq!(integer(1000), res2);
}

#[test]
fn decode_big_integer() {
    // 21> term_to_binary(5130000000).
    // <<131,110,5,0,128,150,197,49,1>>
    let input1 = binary_data(&[131, 110, 5, 0, 128, 150, 197, 49, 1]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(big_integer(5130000000), res1);
    // change term type to 111 (LARGE_BIG_EXT), pad the length value
    let input2 = binary_data(&[131, 111, 0, 0, 0, 5, 0, 128, 150, 197, 49, 1]);
    let res2 = ErlangExtTerm::decode(input2).unwrap();
    assert_eq!(big_integer(5130000000), res2);
}

#[test]
fn decode_negative_integer() {
    // 12> term_to_binary(-1000).
    // <<131,98,255,255,252,24>>
    let input1 = binary_data(&[131, 98, 255, 255, 252, 24]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(integer(-1000), res1);
    // 13> term_to_binary(-1).
    // <<131,98,255,255,255,255>>
    let input2 = binary_data(&[131, 98, 255, 255, 255, 255]);
    let res2 = ErlangExtTerm::decode(input2).unwrap();
    assert_eq!(integer(-1), res2);
}

#[test]
fn decode_positive_float() {
    // 35> term_to_binary(121.7)
    // <<131,70,64,94,108,204,204,204,204,205>>
    let input1 = binary_data(&[131, 70, 64, 94, 108, 204, 204, 204, 204, 205]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(float(121.7), res1);
}

#[test]
fn decode_negative_float() {
    // 36> term_to_binary(-121.7).
    // <<131,70,192,94,108,204,204,204,204,205>>
    let input1 = binary_data(&[131, 70, 192, 94, 108, 204, 204, 204, 204, 205]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(float(-121.7), res1);
}

#[test]
fn decode_binary() {
    // 47> term_to_binary(<<"abc">>).
    // <<131,109,0,0,0,3,97,98,99>>
    let input1 = binary_data(&[131, 109, 0, 0, 0, 3, 97, 98, 99]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(binary("abc"), res1);

    // 49> term_to_binary(<<"abc кириллица"/utf8>>).
    // <<131,109,0,0,0,22,97,98,99,32,208,186,208,184,209,128,208,184,208,187,208,187,208,184,209,134,208,176>>
    let input2 = binary_data(&[
        131, 109, 0, 0, 0, 22, 97, 98, 99, 32, 208, 186, 208, 184, 209, 128, 208, 184, 208, 187,
        208, 187, 208, 184, 209, 134, 208, 176,
    ]);
    let res2 = ErlangExtTerm::decode(input2).unwrap();
    assert_eq!(binary("abc кириллица"), res2);
}

#[test]
fn decode_bit_binary() {
    let input1 = binary_data(&[131, 77, 0, 0, 0, 3, 5, 1, 2, 24]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(bit_binary(vec![1, 2, 3], 5), res1);
}

#[test]
fn decode_pid() {
    // term_to_binary(self()).
    // <<131,88,100,0,13,110,111,110,111,100,101,64,110,111,104,111,115,116,0,0,0,87,0,0,0,0,0,0,0,0>>
    let input1 = binary_data(&[
        131, 88, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111, 115, 116, 0, 0,
        0, 87, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(erl_pid(atom("nonode@nohost"), 87, 0, 0), res1);
}

#[test]
fn decode_v3_port() {
    // term_to_binary(Port).
    // %% this is a local port, so a v3 one
    // <<131,89,100,0,13,110,111,110,111,100,101,64,110,111,104,111,115,116,0,0,0,4,0,0,0,0>>
    let input1 = binary_data(&[
        131, 89, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111, 115, 116, 0, 0,
        0, 4, 0, 0, 0, 0,
    ]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(erl_v3_port(atom("nonode@nohost"), 4, 0), res1);
}

#[test]
fn decode_v4_port() {
    // term_to_binary(Port).
    // <<131,120,100,0,13,110,111,110,111,100,101,64,110,111,104,111,115,116,0,0,0,0,0,0,0,4,0,0,0,0>>
    let input1 = binary_data(&[
        131, 120, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111, 115, 116, 0, 0,
        0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
    ]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(erl_v4_port(atom("nonode@nohost"), 4, 0), res1);
}

//
// Helpers
//

fn binary_data<T>(bytes: T) -> Box<Cursor<T>> {
    Box::new(Cursor::new(bytes))
}

fn atom(s: &str) -> ErlangExtTerm {
    ErlangExtTerm::Atom(s.to_string())
}

fn small_integer(i: u8) -> ErlangExtTerm {
    ErlangExtTerm::SmallInteger(i)
}

fn integer(i: i32) -> ErlangExtTerm {
    ErlangExtTerm::Integer(i)
}

fn big_integer(i: i64) -> ErlangExtTerm {
    ErlangExtTerm::BigInteger(i.to_bigint().unwrap())
}

fn float(i: f64) -> ErlangExtTerm {
    ErlangExtTerm::Float(OrderedFloat::<f64>(i as f64))
}

fn binary(s: &str) -> ErlangExtTerm {
    ErlangExtTerm::Binary(s.as_bytes().to_vec())
}

fn bit_binary(data: Vec<u8>, tail_len: u8) -> ErlangExtTerm {
    ErlangExtTerm::BitBinary(data, tail_len)
}

fn erl_pid(node: ErlangExtTerm, id: u32, serial: u32, creation: u32) -> ErlangExtTerm {
    ErlangExtTerm::ErlPid(ErlPid {
        node: TryInto::<Atom>::try_into(node).unwrap(),
        id,
        serial,
        creation,
    })
}

fn erl_v3_port(node: ErlangExtTerm, id: u32, creation: u32) -> ErlangExtTerm {
    ErlangExtTerm::ErlV3Port(ErlV3Port {
        node: TryInto::<Atom>::try_into(node).unwrap(),
        id,
        creation,
    })
}

fn erl_v4_port(node: ErlangExtTerm, id: u64, creation: u32) -> ErlangExtTerm {
    ErlangExtTerm::ErlV4Port(ErlV4Port {
        node: TryInto::<Atom>::try_into(node).unwrap(),
        id,
        creation,
    })
}

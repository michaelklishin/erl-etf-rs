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

#[test]
fn decode_small_tuple_of_integers() {
    // term_to_binary({1, 2, 3, 4}).
    // <<131,104,4,97,1,97,2,97,3,97,4>>
    let input1 = binary_data(&[131, 104, 4, 97, 1, 97, 2, 97, 3, 97, 4]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(tuple_of_u8(vec![1, 2, 3, 4]), res1);
}

#[test]
fn decode_small_tuple_of_binaries() {
    // term_to_binary({<<"aa">>, <<"bbb">>, <<"c">>, <<"dddd">>}).
    // <<131,104,4,109,0,0,0,2,97,97,109,0,0,0,3,98,98,98,109,0,...>>
    let input1 = binary_data(&[
        131,104,4,109,0,0,0,2,97,97,109,0,0,0,3,98,98,98,109,0,0,0,1,99,109,0,0,0,4,100,100,100,100
    ]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    assert_eq!(tuple_of_binaries(vec!["aa", "bbb", "c", "dddd"]), res1);

    // term_to_binary({<<"erlang">>, <<"rust">>}).
    // <<131,104,2,109,0,0,0,6,101,114,108,97,110,103,109,0,0,0,4,114,117,115,116>>
    let input2 = binary_data(&[
        131,104,2,109,0,0,0,6,101,114,108,97,110,103,109,0,0,0,4,114,117,115,116
    ]);
    let res2 = ErlangExtTerm::decode(input2).unwrap();
    assert_eq!(tuple_of_binaries(vec!["erlang", "rust"]), res2);
}

#[test]
fn decode_large_tuple_of_integers() {
    // term_to_binary(…)
    // <<131,105,0,0,3,232,97,0,97,1,97,2,97,3,97,4,97,5,97,6,97,...>>
    let input1 = binary_data(&[
        131,105,0,0,3,232,97,0,97,1,97,2,97,3,97,4,97,5,97,6,97,7,97,8,97,9,97,10,97,
        11,97,12,97,13,97,14,97,15,97,16,97,17,97,18,97,19,97,20,97,21,97,22,97,23,
        97,24,97,25,97,26,97,27,97,28,97,29,97,30,97,31,97,32,97,33,97,34,97,35,97,
        36,97,37,97,38,97,39,97,40,97,41,97,42,97,43,97,44,97,45,97,46,97,47,97,48,
        97,49,97,50,97,51,97,52,97,53,97,54,97,55,97,56,97,57,97,58,97,59,97,60,97,
        61,97,62,97,63,97,64,97,65,97,66,97,67,97,68,97,69,97,70,97,71,97,72,97,73,
        97,74,97,75,97,76,97,77,97,78,97,79,97,80,97,81,97,82,97,83,97,84,97,85,97,
        86,97,87,97,88,97,89,97,90,97,91,97,92,97,93,97,94,97,95,97,96,97,97,97,98,
        97,99,97,100,97,101,97,102,97,103,97,104,97,105,97,106,97,107,97,108,97,109,
        97,110,97,111,97,112,97,113,97,114,97,115,97,116,97,117,97,118,97,119,97,120,
        97,121,97,122,97,123,97,124,97,125,97,126,97,127,97,128,97,129,97,130,97,131,
        97,132,97,133,97,134,97,135,97,136,97,137,97,138,97,139,97,140,97,141,97,142,
        97,143,97,144,97,145,97,146,97,147,97,148,97,149,97,150,97,151,97,152,97,153,
        97,154,97,155,97,156,97,157,97,158,97,159,97,160,97,161,97,162,97,163,97,164,
        97,165,97,166,97,167,97,168,97,169,97,170,97,171,97,172,97,173,97,174,97,175,
        97,176,97,177,97,178,97,179,97,180,97,181,97,182,97,183,97,184,97,185,97,186,
        97,187,97,188,97,189,97,190,97,191,97,192,97,193,97,194,97,195,97,196,97,197,
        97,198,97,199,97,200,97,201,97,202,97,203,97,204,97,205,97,206,97,207,97,208,
        97,209,97,210,97,211,97,212,97,213,97,214,97,215,97,216,97,217,97,218,97,219,
        97,220,97,221,97,222,97,223,97,224,97,225,97,226,97,227,97,228,97,229,97,230,
        97,231,97,232,97,233,97,234,97,235,97,236,97,237,97,238,97,239,97,240,97,241,
        97,242,97,243,97,244,97,245,97,246,97,247,97,248,97,249,97,250,97,251,97,252,
        97,253,97,254,97,255,98,0,0,1,0,98,0,0,1,1,98,0,0,1,2,98,0,0,1,3,98,0,0,1,4,
        98,0,0,1,5,98,0,0,1,6,98,0,0,1,7,98,0,0,1,8,98,0,0,1,9,98,0,0,1,10,98,0,0,1,
        11,98,0,0,1,12,98,0,0,1,13,98,0,0,1,14,98,0,0,1,15,98,0,0,1,16,98,0,0,1,17,
        98,0,0,1,18,98,0,0,1,19,98,0,0,1,20,98,0,0,1,21,98,0,0,1,22,98,0,0,1,23,98,0,
        0,1,24,98,0,0,1,25,98,0,0,1,26,98,0,0,1,27,98,0,0,1,28,98,0,0,1,29,98,0,0,1,
        30,98,0,0,1,31,98,0,0,1,32,98,0,0,1,33,98,0,0,1,34,98,0,0,1,35,98,0,0,1,36,
        98,0,0,1,37,98,0,0,1,38,98,0,0,1,39,98,0,0,1,40,98,0,0,1,41,98,0,0,1,42,98,0,
        0,1,43,98,0,0,1,44,98,0,0,1,45,98,0,0,1,46,98,0,0,1,47,98,0,0,1,48,98,0,0,1,
        49,98,0,0,1,50,98,0,0,1,51,98,0,0,1,52,98,0,0,1,53,98,0,0,1,54,98,0,0,1,55,
        98,0,0,1,56,98,0,0,1,57,98,0,0,1,58,98,0,0,1,59,98,0,0,1,60,98,0,0,1,61,98,0,
        0,1,62,98,0,0,1,63,98,0,0,1,64,98,0,0,1,65,98,0,0,1,66,98,0,0,1,67,98,0,0,1,
        68,98,0,0,1,69,98,0,0,1,70,98,0,0,1,71,98,0,0,1,72,98,0,0,1,73,98,0,0,1,74,
        98,0,0,1,75,98,0,0,1,76,98,0,0,1,77,98,0,0,1,78,98,0,0,1,79,98,0,0,1,80,98,0,
        0,1,81,98,0,0,1,82,98,0,0,1,83,98,0,0,1,84,98,0,0,1,85,98,0,0,1,86,98,0,0,1,
        87,98,0,0,1,88,98,0,0,1,89,98,0,0,1,90,98,0,0,1,91,98,0,0,1,92,98,0,0,1,93,
        98,0,0,1,94,98,0,0,1,95,98,0,0,1,96,98,0,0,1,97,98,0,0,1,98,98,0,0,1,99,98,0,
        0,1,100,98,0,0,1,101,98,0,0,1,102,98,0,0,1,103,98,0,0,1,104,98,0,0,1,105,98,
        0,0,1,106,98,0,0,1,107,98,0,0,1,108,98,0,0,1,109,98,0,0,1,110,98,0,0,1,111,
        98,0,0,1,112,98,0,0,1,113,98,0,0,1,114,98,0,0,1,115,98,0,0,1,116,98,0,0,1,
        117,98,0,0,1,118,98,0,0,1,119,98,0,0,1,120,98,0,0,1,121,98,0,0,1,122,98,0,0,
        1,123,98,0,0,1,124,98,0,0,1,125,98,0,0,1,126,98,0,0,1,127,98,0,0,1,128,98,0,
        0,1,129,98,0,0,1,130,98,0,0,1,131,98,0,0,1,132,98,0,0,1,133,98,0,0,1,134,98,
        0,0,1,135,98,0,0,1,136,98,0,0,1,137,98,0,0,1,138,98,0,0,1,139,98,0,0,1,140,
        98,0,0,1,141,98,0,0,1,142,98,0,0,1,143,98,0,0,1,144,98,0,0,1,145,98,0,0,1,
        146,98,0,0,1,147,98,0,0,1,148,98,0,0,1,149,98,0,0,1,150,98,0,0,1,151,98,0,0,
        1,152,98,0,0,1,153,98,0,0,1,154,98,0,0,1,155,98,0,0,1,156,98,0,0,1,157,98,0,
        0,1,158,98,0,0,1,159,98,0,0,1,160,98,0,0,1,161,98,0,0,1,162,98,0,0,1,163,98,
        0,0,1,164,98,0,0,1,165,98,0,0,1,166,98,0,0,1,167,98,0,0,1,168,98,0,0,1,169,
        98,0,0,1,170,98,0,0,1,171,98,0,0,1,172,98,0,0,1,173,98,0,0,1,174,98,0,0,1,
        175,98,0,0,1,176,98,0,0,1,177,98,0,0,1,178,98,0,0,1,179,98,0,0,1,180,98,0,0,
        1,181,98,0,0,1,182,98,0,0,1,183,98,0,0,1,184,98,0,0,1,185,98,0,0,1,186,98,0,
        0,1,187,98,0,0,1,188,98,0,0,1,189,98,0,0,1,190,98,0,0,1,191,98,0,0,1,192,98,
        0,0,1,193,98,0,0,1,194,98,0,0,1,195,98,0,0,1,196,98,0,0,1,197,98,0,0,1,198,
        98,0,0,1,199,98,0,0,1,200,98,0,0,1,201,98,0,0,1,202,98,0,0,1,203,98,0,0,1,
        204,98,0,0,1,205,98,0,0,1,206,98,0,0,1,207,98,0,0,1,208,98,0,0,1,209,98,0,0,
        1,210,98,0,0,1,211,98,0,0,1,212,98,0,0,1,213,98,0,0,1,214,98,0,0,1,215,98,0,
        0,1,216,98,0,0,1,217,98,0,0,1,218,98,0,0,1,219,98,0,0,1,220,98,0,0,1,221,98,
        0,0,1,222,98,0,0,1,223,98,0,0,1,224,98,0,0,1,225,98,0,0,1,226,98,0,0,1,227,
        98,0,0,1,228,98,0,0,1,229,98,0,0,1,230,98,0,0,1,231,98,0,0,1,232,98,0,0,1,
        233,98,0,0,1,234,98,0,0,1,235,98,0,0,1,236,98,0,0,1,237,98,0,0,1,238,98,0,0,
        1,239,98,0,0,1,240,98,0,0,1,241,98,0,0,1,242,98,0,0,1,243,98,0,0,1,244,98,0,
        0,1,245,98,0,0,1,246,98,0,0,1,247,98,0,0,1,248,98,0,0,1,249,98,0,0,1,250,98,
        0,0,1,251,98,0,0,1,252,98,0,0,1,253,98,0,0,1,254,98,0,0,1,255,98,0,0,2,0,98,
        0,0,2,1,98,0,0,2,2,98,0,0,2,3,98,0,0,2,4,98,0,0,2,5,98,0,0,2,6,98,0,0,2,7,98,
        0,0,2,8,98,0,0,2,9,98,0,0,2,10,98,0,0,2,11,98,0,0,2,12,98,0,0,2,13,98,0,0,2,
        14,98,0,0,2,15,98,0,0,2,16,98,0,0,2,17,98,0,0,2,18,98,0,0,2,19,98,0,0,2,20,
        98,0,0,2,21,98,0,0,2,22,98,0,0,2,23,98,0,0,2,24,98,0,0,2,25,98,0,0,2,26,98,0,
        0,2,27,98,0,0,2,28,98,0,0,2,29,98,0,0,2,30,98,0,0,2,31,98,0,0,2,32,98,0,0,2,
        33,98,0,0,2,34,98,0,0,2,35,98,0,0,2,36,98,0,0,2,37,98,0,0,2,38,98,0,0,2,39,
        98,0,0,2,40,98,0,0,2,41,98,0,0,2,42,98,0,0,2,43,98,0,0,2,44,98,0,0,2,45,98,0,
        0,2,46,98,0,0,2,47,98,0,0,2,48,98,0,0,2,49,98,0,0,2,50,98,0,0,2,51,98,0,0,2,
        52,98,0,0,2,53,98,0,0,2,54,98,0,0,2,55,98,0,0,2,56,98,0,0,2,57,98,0,0,2,58,
        98,0,0,2,59,98,0,0,2,60,98,0,0,2,61,98,0,0,2,62,98,0,0,2,63,98,0,0,2,64,98,0,
        0,2,65,98,0,0,2,66,98,0,0,2,67,98,0,0,2,68,98,0,0,2,69,98,0,0,2,70,98,0,0,2,
        71,98,0,0,2,72,98,0,0,2,73,98,0,0,2,74,98,0,0,2,75,98,0,0,2,76,98,0,0,2,77,
        98,0,0,2,78,98,0,0,2,79,98,0,0,2,80,98,0,0,2,81,98,0,0,2,82,98,0,0,2,83,98,0,
        0,2,84,98,0,0,2,85,98,0,0,2,86,98,0,0,2,87,98,0,0,2,88,98,0,0,2,89,98,0,0,2,
        90,98,0,0,2,91,98,0,0,2,92,98,0,0,2,93,98,0,0,2,94,98,0,0,2,95,98,0,0,2,96,
        98,0,0,2,97,98,0,0,2,98,98,0,0,2,99,98,0,0,2,100,98,0,0,2,101,98,0,0,2,102,
        98,0,0,2,103,98,0,0,2,104,98,0,0,2,105,98,0,0,2,106,98,0,0,2,107,98,0,0,2,
        108,98,0,0,2,109,98,0,0,2,110,98,0,0,2,111,98,0,0,2,112,98,0,0,2,113,98,0,0,
        2,114,98,0,0,2,115,98,0,0,2,116,98,0,0,2,117,98,0,0,2,118,98,0,0,2,119,98,0,
        0,2,120,98,0,0,2,121,98,0,0,2,122,98,0,0,2,123,98,0,0,2,124,98,0,0,2,125,98,
        0,0,2,126,98,0,0,2,127,98,0,0,2,128,98,0,0,2,129,98,0,0,2,130,98,0,0,2,131,
        98,0,0,2,132,98,0,0,2,133,98,0,0,2,134,98,0,0,2,135,98,0,0,2,136,98,0,0,2,
        137,98,0,0,2,138,98,0,0,2,139,98,0,0,2,140,98,0,0,2,141,98,0,0,2,142,98,0,0,
        2,143,98,0,0,2,144,98,0,0,2,145,98,0,0,2,146,98,0,0,2,147,98,0,0,2,148,98,0,
        0,2,149,98,0,0,2,150,98,0,0,2,151,98,0,0,2,152,98,0,0,2,153,98,0,0,2,154,98,
        0,0,2,155,98,0,0,2,156,98,0,0,2,157,98,0,0,2,158,98,0,0,2,159,98,0,0,2,160,
        98,0,0,2,161,98,0,0,2,162,98,0,0,2,163,98,0,0,2,164,98,0,0,2,165,98,0,0,2,
        166,98,0,0,2,167,98,0,0,2,168,98,0,0,2,169,98,0,0,2,170,98,0,0,2,171,98,0,0,
        2,172,98,0,0,2,173,98,0,0,2,174,98,0,0,2,175,98,0,0,2,176,98,0,0,2,177,98,0,
        0,2,178,98,0,0,2,179,98,0,0,2,180,98,0,0,2,181,98,0,0,2,182,98,0,0,2,183,98,
        0,0,2,184,98,0,0,2,185,98,0,0,2,186,98,0,0,2,187,98,0,0,2,188,98,0,0,2,189,
        98,0,0,2,190,98,0,0,2,191,98,0,0,2,192,98,0,0,2,193,98,0,0,2,194,98,0,0,2,
        195,98,0,0,2,196,98,0,0,2,197,98,0,0,2,198,98,0,0,2,199,98,0,0,2,200,98,0,0,
        2,201,98,0,0,2,202,98,0,0,2,203,98,0,0,2,204,98,0,0,2,205,98,0,0,2,206,98,0,
        0,2,207,98,0,0,2,208,98,0,0,2,209,98,0,0,2,210,98,0,0,2,211,98,0,0,2,212,98,
        0,0,2,213,98,0,0,2,214,98,0,0,2,215,98,0,0,2,216,98,0,0,2,217,98,0,0,2,218,
        98,0,0,2,219,98,0,0,2,220,98,0,0,2,221,98,0,0,2,222,98,0,0,2,223,98,0,0,2,
        224,98,0,0,2,225,98,0,0,2,226,98,0,0,2,227,98,0,0,2,228,98,0,0,2,229,98,0,0,
        2,230,98,0,0,2,231,98,0,0,2,232,98,0,0,2,233,98,0,0,2,234,98,0,0,2,235,98,0,
        0,2,236,98,0,0,2,237,98,0,0,2,238,98,0,0,2,239,98,0,0,2,240,98,0,0,2,241,98,
        0,0,2,242,98,0,0,2,243,98,0,0,2,244,98,0,0,2,245,98,0,0,2,246,98,0,0,2,247,
        98,0,0,2,248,98,0,0,2,249,98,0,0,2,250,98,0,0,2,251,98,0,0,2,252,98,0,0,2,
        253,98,0,0,2,254,98,0,0,2,255,98,0,0,3,0,98,0,0,3,1,98,0,0,3,2,98,0,0,3,3,98,
        0,0,3,4,98,0,0,3,5,98,0,0,3,6,98,0,0,3,7,98,0,0,3,8,98,0,0,3,9,98,0,0,3,10,
        98,0,0,3,11,98,0,0,3,12,98,0,0,3,13,98,0,0,3,14,98,0,0,3,15,98,0,0,3,16,98,0,
        0,3,17,98,0,0,3,18,98,0,0,3,19,98,0,0,3,20,98,0,0,3,21,98,0,0,3,22,98,0,0,3,
        23,98,0,0,3,24,98,0,0,3,25,98,0,0,3,26,98,0,0,3,27,98,0,0,3,28,98,0,0,3,29,
        98,0,0,3,30,98,0,0,3,31,98,0,0,3,32,98,0,0,3,33,98,0,0,3,34,98,0,0,3,35,98,0,
        0,3,36,98,0,0,3,37,98,0,0,3,38,98,0,0,3,39,98,0,0,3,40,98,0,0,3,41,98,0,0,3,
        42,98,0,0,3,43,98,0,0,3,44,98,0,0,3,45,98,0,0,3,46,98,0,0,3,47,98,0,0,3,48,
        98,0,0,3,49,98,0,0,3,50,98,0,0,3,51,98,0,0,3,52,98,0,0,3,53,98,0,0,3,54,98,0,
        0,3,55,98,0,0,3,56,98,0,0,3,57,98,0,0,3,58,98,0,0,3,59,98,0,0,3,60,98,0,0,3,
        61,98,0,0,3,62,98,0,0,3,63,98,0,0,3,64,98,0,0,3,65,98,0,0,3,66,98,0,0,3,67,
        98,0,0,3,68,98,0,0,3,69,98,0,0,3,70,98,0,0,3,71,98,0,0,3,72,98,0,0,3,73,98,0,
        0,3,74,98,0,0,3,75,98,0,0,3,76,98,0,0,3,77,98,0,0,3,78,98,0,0,3,79,98,0,0,3,
        80,98,0,0,3,81,98,0,0,3,82,98,0,0,3,83,98,0,0,3,84,98,0,0,3,85,98,0,0,3,86,
        98,0,0,3,87,98,0,0,3,88,98,0,0,3,89,98,0,0,3,90,98,0,0,3,91,98,0,0,3,92,98,0,
        0,3,93,98,0,0,3,94,98,0,0,3,95,98,0,0,3,96,98,0,0,3,97,98,0,0,3,98,98,0,0,3,
        99,98,0,0,3,100,98,0,0,3,101,98,0,0,3,102,98,0,0,3,103,98,0,0,3,104,98,0,0,3,
        105,98,0,0,3,106,98,0,0,3,107,98,0,0,3,108,98,0,0,3,109,98,0,0,3,110,98,0,0,
        3,111,98,0,0,3,112,98,0,0,3,113,98,0,0,3,114,98,0,0,3,115,98,0,0,3,116,98,0,
        0,3,117,98,0,0,3,118,98,0,0,3,119,98,0,0,3,120,98,0,0,3,121,98,0,0,3,122,98,
        0,0,3,123,98,0,0,3,124,98,0,0,3,125,98,0,0,3,126,98,0,0,3,127,98,0,0,3,128,
        98,0,0,3,129,98,0,0,3,130,98,0,0,3,131,98,0,0,3,132,98,0,0,3,133,98,0,0,3,
        134,98,0,0,3,135,98,0,0,3,136,98,0,0,3,137,98,0,0,3,138,98,0,0,3,139,98,0,0,
        3,140,98,0,0,3,141,98,0,0,3,142,98,0,0,3,143,98,0,0,3,144,98,0,0,3,145,98,0,
        0,3,146,98,0,0,3,147,98,0,0,3,148,98,0,0,3,149,98,0,0,3,150,98,0,0,3,151,98,
        0,0,3,152,98,0,0,3,153,98,0,0,3,154,98,0,0,3,155,98,0,0,3,156,98,0,0,3,157,
        98,0,0,3,158,98,0,0,3,159,98,0,0,3,160,98,0,0,3,161,98,0,0,3,162,98,0,0,3,
        163,98,0,0,3,164,98,0,0,3,165,98,0,0,3,166,98,0,0,3,167,98,0,0,3,168,98,0,0,
        3,169,98,0,0,3,170,98,0,0,3,171,98,0,0,3,172,98,0,0,3,173,98,0,0,3,174,98,0,
        0,3,175,98,0,0,3,176,98,0,0,3,177,98,0,0,3,178,98,0,0,3,179,98,0,0,3,180,98,
        0,0,3,181,98,0,0,3,182,98,0,0,3,183,98,0,0,3,184,98,0,0,3,185,98,0,0,3,186,
        98,0,0,3,187,98,0,0,3,188,98,0,0,3,189,98,0,0,3,190,98,0,0,3,191,98,0,0,3,
        192,98,0,0,3,193,98,0,0,3,194,98,0,0,3,195,98,0,0,3,196,98,0,0,3,197,98,0,0,
        3,198,98,0,0,3,199,98,0,0,3,200,98,0,0,3,201,98,0,0,3,202,98,0,0,3,203,98,0,
        0,3,204,98,0,0,3,205,98,0,0,3,206,98,0,0,3,207,98,0,0,3,208,98,0,0,3,209,98,
        0,0,3,210,98,0,0,3,211,98,0,0,3,212,98,0,0,3,213,98,0,0,3,214,98,0,0,3,215,
        98,0,0,3,216,98,0,0,3,217,98,0,0,3,218,98,0,0,3,219,98,0,0,3,220,98,0,0,3,
        221,98,0,0,3,222,98,0,0,3,223,98,0,0,3,224,98,0,0,3,225,98,0,0,3,226,98,0,0,
        3,227,98,0,0,3,228,98,0,0,3,229,98,0,0,3,230,98,0,0,3,231
    ]);
    let res1 = ErlangExtTerm::decode(input1).unwrap();
    let t1 =  TryInto::<Tuple>::try_into(res1).unwrap();
    assert_eq!(1000, t1.elements.len());
    assert_eq!(&ErlangExtTerm::SmallInteger(0), t1.elements.first().unwrap());
    assert_eq!(&ErlangExtTerm::Integer(999), t1.elements.last().unwrap());
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

fn tuple_of_u8(vec: Vec<u8>) -> ErlangExtTerm {
    let xs = vec
        .iter()
        .map(|&i| ErlangExtTerm::SmallInteger(i))
        .collect::<Vec<ErlangExtTerm>>();
    ErlangExtTerm::Tuple(Tuple { elements: xs })
}

fn tuple_of_binaries(vec: Vec<&str>) -> ErlangExtTerm {
    let xs = vec
        .iter()
        .map(|&i| ErlangExtTerm::Binary(i.as_bytes().to_vec()))
        .collect::<Vec<ErlangExtTerm>>();
    ErlangExtTerm::Tuple(Tuple { elements: xs })
}
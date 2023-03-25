use num::bigint::Sign;
use std::io;

pub(crate) fn to_sign(i: u8) -> io::Result<Sign> {
    match i {
        0 => Ok(Sign::Plus),
        1 => Ok(Sign::Minus),
        _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
            format!("BigInteger sign must be either 0 or 1, given: {}", i),
        )),
    }
}

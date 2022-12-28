// Erlang's External Term Format documentation guide
// can be found at https://www.erlang.org/doc/apps/erts/erl_ext_dist.html

// Section 12.1
pub(crate) const TERM_FORMAT_VERSION: u8 = 131;
// Section 12.3
#[allow(dead_code)]
pub(crate) const DISTRIBUTION_HEADER: u8 = 68;
// Sections 12.30 through 12.32
pub(crate) const ATOM_UTF8_EXT: u8 = 118;
pub(crate) const SMALL_ATOM_UTF8_EXT: u8 = 119;
pub(crate) const ATOM_EXT: u8 = 100;
#[allow(dead_code)]
pub(crate) const SMALL_ATOM_EXT: u8 = 115;
// Sections 12.27, 12.6
pub(crate) const NEW_FLOAT_EXT: u8 = 70;
// Section 12.4
pub(crate) const SMALL_INTEGER_EXT: u8 = 97;
// Section 12.5
pub(crate) const INTEGER_EXT: u8 = 98;
// Section 12.18
pub(crate) const SMALL_BIG_EXT: u8 = 110;
// Section 12.19
pub(crate) const LARGE_BIG_EXT: u8 = 111;
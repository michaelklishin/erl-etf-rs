use crate::*;

//
// ErlTerm <=> specific term types
//

impl TryInto<Atom> for ErlTerm {
    type Error = ();

    fn try_into(self) -> Result<Atom, Self::Error> {
        match self {
            ErlTerm::Atom(val) => Ok(Atom { name: val }),
            _ => Err(()),
        }
    }
}
impl TryInto<Tuple> for ErlTerm {
    type Error = ();

    fn try_into(self) -> Result<Tuple, Self::Error> {
        match self {
            ErlTerm::Tuple(val) => Ok(Tuple { elements: val.elements }),
            _ => Err(()),
        }
    }
}


//
// Specific term types <=> core Rust types
//

impl From<Atom> for ErlTerm {
    fn from(val: Atom) -> Self {
        ErlTerm::Atom(val.name)
    }
}
impl From<String> for ErlTerm {
    fn from(val: String) -> Self {
        ErlTerm::Atom(val)
    }
}
impl From<&str> for ErlTerm {
    fn from(val: &str) -> Self {
        ErlTerm::Atom(val.to_string())
    }
}

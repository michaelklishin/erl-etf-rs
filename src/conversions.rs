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
impl TryInto<u8> for ErlTerm {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            ErlTerm::SmallInteger(val) => Ok(val),
            ErlTerm::Integer(val) => Ok(val as u8),
            _ => Err(()),
        }
    }
}
impl TryInto<i32> for ErlTerm {
    type Error = ();

    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            ErlTerm::SmallInteger(val) => Ok(val as i32),
            ErlTerm::Integer(val) => Ok(val),
            _ => Err(()),
        }
    }
}
impl TryInto<ErlPid> for ErlTerm {
    type Error = ();

    fn try_into(self) -> Result<ErlPid, Self::Error> {
        match self {
            ErlTerm::Pid(val) => Ok(val),
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
impl TryInto<List> for ErlTerm {
    type Error = ();

    fn try_into(self) -> Result<List, Self::Error> {
        match self {
            ErlTerm::List(val) => Ok(List { elements: val.elements }),
            _ => Err(()),
        }
    }
}
impl TryInto<ImproperList> for ErlTerm {
    type Error = ();

    fn try_into(self) -> Result<ImproperList, Self::Error> {
        match self {
            ErlTerm::ImproperList(val) => Ok(ImproperList {
                elements: val.elements,
                tail: val.tail
            }),
            _ => Err(()),
        }
    }
}
impl TryInto<InternalFun> for ErlTerm {
    type Error = ();

    fn try_into(self) -> Result<InternalFun, Self::Error> {
        match self {
            ErlTerm::InternalFun(val) => Ok(val),
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

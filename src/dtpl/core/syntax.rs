#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Hash)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
pub enum Name {
    Const(String),
    Bound(uint),
    Unquoted(uint),
}

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Hash)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
pub enum Type {
    TPar(Name),
    Fun(Box<Type>, Box<Type>),
}

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Hash)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
pub enum CTerm {
    Inf(Box<ITerm>),
    Lam(Box<CTerm>),
}

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Hash)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
pub enum ITerm {
    Ann(Box<CTerm>, Box<Type>),
    App(Box<ITerm>, Box<CTerm>),
    Var(uint),
    Par(Name),
}

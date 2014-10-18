pub mod sym {
    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Hash)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Sym {
        Con(String),
        // Bnd(uint),
        // UQuo(uint),
    }
}

pub mod typ {
    use super::sym;
    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Hash)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Typ {
        Par(sym::Sym),
        Fun(Box<Typ>, Box<Typ>),
    }
}

pub mod chk {
    use super::chk;
    use super::inf;
    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Hash)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Chk {
        Inf(Box<inf::Inf>),
        Abs(Box<chk::Chk>),
    }
}

pub mod inf {
    use super::chk;
    use super::inf;
    use super::sym;
    use super::typ;
    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Hash)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Inf {
        Ann(chk::Chk, typ::Typ),
        App(Box<inf::Inf>, chk::Chk),
        Var(uint),
        Par(sym::Sym),
    }
}

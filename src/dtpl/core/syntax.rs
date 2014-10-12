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
        Bnd(uint),
        UQuo(uint),
    }
}

pub mod typ {
    use core::syntax::sym;
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
    use core::syntax::chk;
    use core::syntax::inf;
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
    use core::syntax::chk;
    use core::syntax::inf;
    use core::syntax::sym;
    use core::syntax::typ;
    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Hash)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Inf {
        Ann(Box<chk::Chk>, Box<typ::Typ>),
        App(Box<inf::Inf>, Box<chk::Chk>),
        Var(uint),
        Par(sym::Sym),
    }
}

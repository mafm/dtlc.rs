pub mod neu {
    use super::nrm;
    use core::syntax::sym;
    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Neu {
        Par(sym::Sym),
        App(Box<Neu>, Box<nrm::Nrm>),
    }
}

pub mod nrm {
    use super::neu;
    use core::syntax::chk;
    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Nrm {
        Abs(Box<chk::Chk>, Env),
        Neu(Box<neu::Neu>),
    }
    pub type Env = Vec<Nrm>;
}
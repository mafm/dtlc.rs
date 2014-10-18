pub mod neu {
    use super::nrm;
    use super::super::syntax::sym;
    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Neu {
        Par(sym::Sym),
        App(Box<Neu>, nrm::Nrm),
    }
}

pub mod nrm {
    use super::neu;
    use super::super::syntax::chk;
    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Nrm {
        Abs(chk::Chk, Env),
        Neu(Box<neu::Neu>),
    }
    pub type Env = Vec<Nrm>;
}

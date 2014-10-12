pub mod neu {
    use core::domain::nrm;
    use core::syntax::sym;
    #[deriving(Clone)]
    #[deriving(Show)]
    pub enum Neu {
        Par(sym::Sym),
        App(Box<Neu>, Box<nrm::Nrm>),
    }
}

pub mod nrm {
    use core::normal;
    use core::domain::neu;
    use core::syntax::chk;
    #[deriving(Clone)]
    #[deriving(Show)]
    pub enum Nrm {
        Abs(Box<chk::Chk>, normal::Env),
        Neu(Box<neu::Neu>),
    }
}

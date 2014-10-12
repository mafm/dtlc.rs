pub mod neu {
    use core::model::nrm;
    use core::syntax::sym;
    #[deriving(Clone)]
    #[deriving(Show)]
    pub enum Neu {
        Par(sym::Sym),
        App(Box<Neu>, Box<nrm::Nrm>),
    }
}

pub mod nrm {
    use core::eval;
    use core::model::neu;
    use core::syntax::chk;
    #[deriving(Clone)]
    #[deriving(Show)]
    pub enum Nrm {
        Abs(Box<chk::Chk>, eval::Env),
        Neu(Box<neu::Neu>),
    }
}

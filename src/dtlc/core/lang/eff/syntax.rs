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
    pub enum Val {
        Par(sym::Sym),                              // X
        Car(Box<Cmp>),                              // { C }
    }

    #[deriving(Clone)]
    #[deriving(Eq)]
    #[deriving(Hash)]
    #[deriving(Ord)]
    #[deriving(PartialEq)]
    #[deriving(PartialOrd)]
    #[deriving(Show)]
    pub enum Cmp {
        Arr(Box<Val>, Box<Cmp>),                    // V → C
        Mon(Box<Val>),                              // []V
    }
}

pub mod trm {
    pub mod chk {
        use super::inf;

        #[deriving(Clone)]
        #[deriving(Eq)]
        #[deriving(Hash)]
        #[deriving(Ord)]
        #[deriving(PartialEq)]
        #[deriving(PartialOrd)]
        #[deriving(Show)]
        pub enum Val {
            Eff(Box<self::Cmp>),                    // eff cc
            Inf(inf::Val),                          // inf iv
        }

        #[deriving(Clone)]
        #[deriving(Eq)]
        #[deriving(Hash)]
        #[deriving(Ord)]
        #[deriving(PartialEq)]
        #[deriving(PartialOrd)]
        #[deriving(Show)]
        pub enum Cmp {
            Lam(Box<self::Cmp>),                    // λ x ⇒ cc
            Ret(self::Val),                         // ret cv
        }
    }

    pub mod inf {
        use super::chk;
        use super::super::sym;
        use super::super::typ;

        #[deriving(Clone)]
        #[deriving(Eq)]
        #[deriving(Hash)]
        #[deriving(Ord)]
        #[deriving(PartialEq)]
        #[deriving(PartialOrd)]
        #[deriving(Show)]
        pub enum Val {
            Var(uint),                              // x
            Par(sym::Sym),                          // c
        }

        #[deriving(Clone)]
        #[deriving(Eq)]
        #[deriving(Hash)]
        #[deriving(Ord)]
        #[deriving(PartialEq)]
        #[deriving(PartialOrd)]
        #[deriving(Show)]
        pub enum Cmp {
            Ann(chk::Cmp, typ::Cmp),                // (cc : C)
            App(Box<self::Cmp>, chk::Val),          // ic · cv
            Obs(self::Val),                         // obs iv
            Seq(Box<self::Cmp>, chk::Cmp),          // seq x ⇐ ic in cc
        }
    }

}

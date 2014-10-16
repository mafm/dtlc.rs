#![feature(phase)]

extern crate core;
#[phase(plugin)]
extern crate quickcheck_macros;
extern crate quickcheck;

extern crate dtpl;

mod tm {
    use dtpl::core::syntax::{
        chk,
        inf,
        // sym,
    };

    // ff = λf. λt. f
    pub fn mk_ff() -> chk::Chk {
        chk::Abs( // λf.
            box chk::Abs( // λt.
                box chk::Inf(
                    box inf::Var(1) // f
                )
            )
        )
    }

    // tt = λf. λt. t
    pub fn mk_tt() -> chk::Chk {
        chk::Abs( // λf.
            box chk::Abs( // λt.
                box chk::Inf(
                    box inf::Var(0) // t
                )
            )
        )
    }

    // not = λb. b tt ff
    pub fn mk_not() -> chk::Chk {
        chk::Abs( // λb.
            box chk::Inf(
                box inf::App( // ((b tt) ff)
                    box inf::App( // (b tt)
                        box inf::Var(0), // b
                        box mk_tt() // tt
                    ),
                    box mk_ff() // ff
                )
            )
        )
    }

    // and = λb1. λb2. b1 ff b2
    pub fn mk_and() -> chk::Chk {
        chk::Abs( // λb1.
            box chk::Abs( // λb2.
                box chk::Inf(
                    box inf::App( // ((b1 ff) b2)
                        box inf::App( // (b1 ff)
                            box inf::Var(1), // b1
                            box mk_ff() // ff
                        ),
                        box chk::Inf(
                            box inf::Var(0) // b2
                        )
                    )
                )
            )
        )
    }

    // or  = λb1. λb1. b1 b2 tt
    pub fn mk_or() -> chk::Chk {
        chk::Abs( // λb1.
            box chk::Abs( // λb2.
                box chk::Inf(
                    box inf::App( // ((b1 b2) tt)
                        box inf::App( // (b1 b2)
                            box inf::Var(1), // b1
                            box chk::Inf(
                                box inf::Var(0) // b2
                            )
                        ),
                        box mk_tt() // tt
                    )
                )
            )
        )
    }

    // λc. λaf. λat. c af at
    #[allow(dead_code)]
    pub fn mk_if() -> chk::Chk {
        chk::Abs( // λc.
            box chk::Abs( // λat.
                box chk::Abs( // λaf.
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Var(2), // c
                                box chk::Inf(
                                    box inf::Var(1) // at
                                )
                            ),
                            box chk::Inf(
                                box inf::Var(0) // af
                            )
                        )
                    )
                )
            )
        )
    }

}

mod wrap {
    use core::fmt;
    use quickcheck as qchk;
    use dtpl::core::syntax::{
        chk,
    };
    use super::tm;

    pub struct Wrap<A:Clone,T>(A);

    impl<A,T> Wrap<A,T> where
        A:Clone,
    {
        pub fn unwrap(&self) -> A {
            let Wrap(ref inner) = *self;
            inner.clone()
        }
    }

    impl<A,T> Clone for Wrap<A,T> where
        A:Clone,
    {
        fn clone(&self) -> Wrap<A,T> {
            Wrap(self.unwrap())
        }
    }

    impl<A,T> fmt::Show for Wrap<A,T> where
        A:Clone,
        A:fmt::Show,
    {
        fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
            self.unwrap().fmt(f)
        }
    }

    struct PhBool;

    pub type Bool = Wrap<chk::Chk,PhBool>;

    impl qchk::Arbitrary for Bool {
        fn arbitrary<G:qchk::Gen>(g:&mut G) -> Bool {
            Wrap(
                match qchk::Arbitrary::arbitrary(g) {
                    false => tm::mk_ff(),
                    true  => tm::mk_tt(),
                }
            )
        }
    }

}

mod test {
    use dtpl::core::domain::{
        nrm,
    };
    use dtpl::core::normal;
    use dtpl::core::syntax::{
        chk,
        inf,
        sym,
        typ,
    };
    use super::tm;
    use super::wrap::{
        Bool,
    };

    fn val_eq(lhs:&nrm::Nrm, rhs:&nrm::Nrm) -> bool {
        match (lhs, rhs) {
            (&nrm::Abs(ref c1, _), &nrm::Abs(ref c2, _)) => { c1 == c2 },
            _ => { lhs == rhs }
        }
    }

    //// axioms from http://ncatlab.org/nlab/show/Boolean+algebra

    // and a tt == a
    #[quickcheck]
    fn qc_bool_and_tt(wa:Bool) -> bool {
        let a = wa.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box tm::mk_tt()
                )
            );
        let trhs = a;
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // or a ff == a
    #[quickcheck]
    fn qc_bool_or_ff(wa:Bool) -> bool {
        let a = wa.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box tm::mk_ff()
                )
            );
        let trhs = a.clone();
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // and a (and b c) == and (and a b) c
    #[quickcheck]
    fn qc_bool_and_asc(wa:Bool, wb:Bool, wc:Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let c = wc.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box tm::mk_and(),
                                    box typ::Par(sym::Con(String::from_str("*")))
                                ),
                                box b.clone()
                            ),
                            box c.clone()
                        )
                    )
                )
            );
        let trhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box chk::Inf(
                            box inf::App(
                                box inf::App(
                                    box inf::Ann(
                                        box tm::mk_and(),
                                        box typ::Par(sym::Con(String::from_str("*")))
                                    ),
                                    box a
                                ),
                                box b
                            )
                        )
                    ),
                    box c
                )
            );
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // or a (or b c) == or (or a b) c
    #[quickcheck]
    fn qc_bool_or_asc(wa:Bool, wb:Bool, wc:Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let c = wc.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box tm::mk_or(),
                                    box typ::Par(sym::Con(String::from_str("*")))
                                ),
                                box b.clone()
                            ),
                            box c.clone()
                        )
                    )
                )
            );
        let trhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box chk::Inf(
                            box inf::App(
                                box inf::App(
                                    box inf::Ann(
                                        box tm::mk_or(),
                                        box typ::Par(sym::Con(String::from_str("*")))
                                    ),
                                    box a
                                ),
                                box b
                            )
                        )
                    ),
                    box c
                )
            );
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // and a b == and b a
    #[quickcheck]
    fn qc_bool_and_comm(wa:Bool, wb:Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box b.clone()
                )
            );
        let trhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box b
                    ),
                    box a
                )
            );
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // or a b == or b a
    #[quickcheck]
    fn qc_bool_or_comm(wa:Bool, wb:Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box b.clone()
                )
            );
        let trhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box b
                    ),
                    box a
                )
            );
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // and a (or a b) == a
    #[quickcheck]
    fn qc_bool_and_or(wa:Bool, wb:Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box tm::mk_or(),
                                    box typ::Par(sym::Con(String::from_str("*")))
                                ),
                                box a.clone()
                            ),
                            box b
                        )
                    )
                )
            );
        let trhs = a;
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // or a (and a b) = a
    #[quickcheck]
    fn qc_bool_or_and(wa:Bool, wb:Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box tm::mk_and(),
                                    box typ::Par(sym::Con(String::from_str("*")))
                                ),
                                box a.clone()
                            ),
                            box b
                        )
                    )
                )
            );
        let trhs = a;
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // and a (or b c) == or (and a b) (and a c)
    #[quickcheck]
    fn qc_bool_and_or_dist(wa:Bool, wb:Bool, wc:Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let c = wc.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box tm::mk_or(),
                                    box typ::Par(sym::Con(String::from_str("*")))
                                ),
                                box b.clone()
                            ),
                            box c.clone()
                        )
                    )
                )
            );
        let trhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box chk::Inf(
                            box inf::App(
                                box inf::App(
                                    box inf::Ann(
                                        box tm::mk_and(),
                                        box typ::Par(sym::Con(String::from_str("*")))
                                    ),
                                    box a.clone()
                                ),
                                box b
                            )
                        )
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box tm::mk_and(),
                                    box typ::Par(sym::Con(String::from_str("*")))
                                ),
                                box a
                            ),
                            box c
                        )
                    )
                )
            );
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // or a (and b c) == and (or a b) (or a c)
    #[quickcheck]
    fn qc_bool_or_and_dist(wa:Bool, wb:Bool, wc:Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let c = wc.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box tm::mk_and(),
                                    box typ::Par(sym::Con(String::from_str("*")))
                                ),
                                box b.clone()
                            ),
                            box c.clone()
                        )
                    )
                )
            );
        let trhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box chk::Inf(
                            box inf::App(
                                box inf::App(
                                    box inf::Ann(
                                        box tm::mk_or(),
                                        box typ::Par(sym::Con(String::from_str("*")))
                                    ),
                                    box a.clone()
                                ),
                                box b
                            )
                        )
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box tm::mk_or(),
                                    box typ::Par(sym::Con(String::from_str("*")))
                                ),
                                box a
                            ),
                            box c
                        )
                    )
                )
            );
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // and a (not a) == ff
    #[quickcheck]
    fn qc_bool_law_non_contradiction(wa:Bool) -> bool {
        let a = wa.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::Ann(
                                box tm::mk_not(),
                                box typ::Par(sym::Con(String::from_str("*")))
                            ),
                            box a
                        )
                    )
                )
            );
        let trhs = tm::mk_ff();
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // or a (not a) == tt
    #[quickcheck]
    fn qc_bool_law_excluded_middle(wa:Bool) -> bool {
        let a = wa.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box tm::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::Ann(
                                box tm::mk_not(),
                                box typ::Par(sym::Con(String::from_str("*")))
                            ),
                            box a
                        )
                    )
                )
            );
        let trhs = tm::mk_tt();
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

}

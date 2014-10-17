// external crates
extern crate core;
extern crate quickcheck;

// local crates
extern crate dtlc;

pub mod wrap {
    // external mod imports
    use core::fmt;
    use quickcheck as qchk;

    // local mod imports
    use dtlc::core::syntax::{
        chk,
    };

    // custom mod imports
    #[path="../../examples/church.rs"]  // link examples/church
    mod church;

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
                    false => church::mk_ff(),
                    true  => church::mk_tt(),
                }
            )
        }
    }

}

pub mod prop {
    // local mod imports
    use dtlc::core::domain::{
        nrm,
    };
    use dtlc::core::normal;
    use dtlc::core::syntax::{
        chk,
        inf,
        sym,
        typ,
    };
    use super::wrap::{
        Bool,
    };

    // custom mod imports
    #[path="../../examples/church.rs"]
    mod church;

    fn val_eq(lhs:&nrm::Nrm, rhs:&nrm::Nrm) -> bool {
        match (lhs, rhs) {
            (&nrm::Abs(ref c1, _), &nrm::Abs(ref c2, _)) => { c1 == c2 },
            _ => { lhs == rhs }
        }
    }

    //// axioms from http://ncatlab.org/nlab/show/Boolean+algebra

    // and a tt == a
    pub fn bool_and_tt_idn(wa:&Bool) -> bool {
        let a = wa.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box church::mk_tt()
                )
            );
        let trhs = a;
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // or a ff == a
    pub fn bool_or_ff_idn(wa:&Bool) -> bool {
        let a = wa.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box church::mk_ff()
                )
            );
        let trhs = a.clone();
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // and a (and b c) == and (and a b) c
    pub fn bool_and_asc(wa:&Bool, wb:&Bool, wc:&Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let c = wc.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box church::mk_and(),
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
                            box church::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box chk::Inf(
                            box inf::App(
                                box inf::App(
                                    box inf::Ann(
                                        box church::mk_and(),
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
    pub fn bool_or_asc(wa:&Bool, wb:&Bool, wc:&Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let c = wc.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box church::mk_or(),
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
                            box church::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box chk::Inf(
                            box inf::App(
                                box inf::App(
                                    box inf::Ann(
                                        box church::mk_or(),
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
    pub fn bool_and_com(wa:&Bool, wb:&Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_and(),
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
                            box church::mk_and(),
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
    pub fn bool_or_com(wa:&Bool, wb:&Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_or(),
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
                            box church::mk_or(),
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
    pub fn bool_and_or_red(wa:&Bool, wb:&Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box church::mk_or(),
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
    pub fn bool_or_and_red(wa:&Bool, wb:&Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box church::mk_and(),
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
    pub fn bool_and_or_dst(wa:&Bool, wb:&Bool, wc:&Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let c = wc.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box church::mk_or(),
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
                            box church::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box chk::Inf(
                            box inf::App(
                                box inf::App(
                                    box inf::Ann(
                                        box church::mk_and(),
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
                                    box church::mk_and(),
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
    pub fn bool_or_and_dst(wa:&Bool, wb:&Bool, wc:&Bool) -> bool {
        let a = wa.unwrap();
        let b = wb.unwrap();
        let c = wc.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::App(
                                box inf::Ann(
                                    box church::mk_and(),
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
                            box church::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box chk::Inf(
                            box inf::App(
                                box inf::App(
                                    box inf::Ann(
                                        box church::mk_or(),
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
                                    box church::mk_or(),
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
    pub fn bool_law_non_con(wa:&Bool) -> bool {
        let a = wa.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_and(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::Ann(
                                box church::mk_not(),
                                box typ::Par(sym::Con(String::from_str("*")))
                            ),
                            box a
                        )
                    )
                )
            );
        let trhs = church::mk_ff();
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

    // or a (not a) == tt
    pub fn bool_law_exc_mid(wa:&Bool) -> bool {
        let a = wa.unwrap();
        let tlhs =
            chk::Inf(
                box inf::App(
                    box inf::App(
                        box inf::Ann(
                            box church::mk_or(),
                            box typ::Par(sym::Con(String::from_str("*")))
                        ),
                        box a.clone()
                    ),
                    box chk::Inf(
                        box inf::App(
                            box inf::Ann(
                                box church::mk_not(),
                                box typ::Par(sym::Con(String::from_str("*")))
                            ),
                            box a
                        )
                    )
                )
            );
        let trhs = church::mk_tt();
        let vlhs = normal::chk(tlhs, vec![]);
        let vrhs = normal::chk(trhs, vec![]);
        val_eq(&vlhs, &vrhs)
    }

}

mod church {
    // external mod imports
    use quickcheck as qchk;
    use std::rand;

    // local mod imports
    use super::prop;
    use super::wrap;

    // and a tt == a
    #[test]
    fn bool_and_tt_idn() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_and_tt_idn(&wa))
    }

    // or a ff == a
    #[test]
    fn bool_or_ff_idn() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_or_ff_idn(&wa))
    }

    // and a (and b c) == and (and a b) c
    #[test]
    fn bool_and_asc() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wb: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wc: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_and_asc(&wa, &wb, &wc))
    }

    // or a (or b c) == or (or a b) c
    #[test]
    fn bool_or_asc() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wb: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wc: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_or_asc(&wa, &wb, &wc))
    }

    // and a b == and b a
    #[test]
    fn bool_and_com() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wb: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_and_com(&wa, &wb))
    }

    // or a b == or b a
    #[test]
    fn bool_or_com() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wb: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_or_com(&wa, &wb))
    }

    // and a (or a b) == a
    #[test]
    fn bool_and_or_red() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wb: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_and_or_red(&wa, &wb))
    }

    // or a (and a b) = a
    #[test]
    fn bool_or_and_red() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wb: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_or_and_red(&wa, &wb))
    }

    // and a (or b c) == or (and a b) (and a c)
    #[test]
    fn bool_and_or_dst() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wb: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wc: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_and_or_dst(&wa, &wb, &wc))
    }

    // or a (and b c) == and (or a b) (or a c)
    #[test]
    fn bool_or_and_dst() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wb: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        let wc: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_or_and_dst(&wa, &wb, &wc))
    }

    // and a (not a) == ff
    #[test]
    fn bool_law_non_con() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_law_non_con(&wa))
    }

    // or a (not a) == tt
    #[test]
    fn bool_law_exc_mid() -> () {
        let gen = &mut qchk::gen(rand::task_rng(), qchk::DEFAULT_SIZE);
        let wa: wrap::Bool = qchk::Arbitrary::arbitrary(gen);
        assert!(prop::bool_law_exc_mid(&wa))
    }

}

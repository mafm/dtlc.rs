// rust feature flags
#![feature(phase)]

// external imports
extern crate core;
#[phase(plugin)]
extern crate quickcheck_macros;
extern crate quickcheck;

// local import
extern crate dtlc;

// link examples/church
#[path="../examples/church.rs"]
mod church;

mod wrap {
    use core::fmt;
    use quickcheck as qchk;
    use dtlc::core::syntax::{
        chk,
    };
    use super::church;

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

mod test {
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
    use super::church;
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
    #[quickcheck]
    fn qc_bool_or_ff(wa:Bool) -> bool {
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
    #[quickcheck]
    fn qc_bool_and_comm(wa:Bool, wb:Bool) -> bool {
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
    #[quickcheck]
    fn qc_bool_or_comm(wa:Bool, wb:Bool) -> bool {
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
    #[quickcheck]
    fn qc_bool_and_or(wa:Bool, wb:Bool) -> bool {
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
    #[quickcheck]
    fn qc_bool_or_and(wa:Bool, wb:Bool) -> bool {
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
    #[quickcheck]
    fn qc_bool_law_non_contradiction(wa:Bool) -> bool {
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
    #[quickcheck]
    fn qc_bool_law_excluded_middle(wa:Bool) -> bool {
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

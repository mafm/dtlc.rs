extern crate dtlc;

use dtlc::core::syntax::{
    chk,
    inf,
};

// ff = λf. λt. f
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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

// FIXME: rustc complains 'main not function not found' without this
#[allow(dead_code)]
fn main() -> () {}

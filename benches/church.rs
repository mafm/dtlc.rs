// rust feature flags
#![feature(phase)]

// external crates
extern crate core;
extern crate quickcheck;
extern crate "test" as std_test;

// local crates
extern crate dtlc;

// external mod imports
use quickcheck as qchk;
use std::rand;

// local mod imports
use test::{
    prop,
    wrap,
};

// custom mod imports
#[path="../tests/church.rs"]
mod test;

// FIXME: not a good bench; probably too allocation heavy; better than nothing
#[bench]
fn bench(b:&mut std_test::Bencher) -> () {
    let rng = rand::task_rng();
    let gen = &mut qchk::gen(rng, qchk::DEFAULT_SIZE);
    let wa: &wrap::Bool = &qchk::Arbitrary::arbitrary(gen);
    let wb: &wrap::Bool = &qchk::Arbitrary::arbitrary(gen);
    let wc: &wrap::Bool = &qchk::Arbitrary::arbitrary(gen);
    let task = || {
        prop::bool_and_tt(wa);
        prop::bool_or_ff(wa);
        prop::bool_and_asc(wa, wb, wc);
        prop::bool_or_asc(wa, wb, wc);
        prop::bool_and_comm(wa, wb);
        prop::bool_or_comm(wa, wb);
        prop::bool_and_or_red(wa, wb);
        prop::bool_or_and_red(wa, wb);
        prop::bool_and_or_dist(wa, wb, wc);
        prop::bool_or_and_dist(wa, wb, wc);
        prop::bool_law_non_contradiction(wa);
        prop::bool_law_excluded_middle(wa);
    };
    b.iter(task);
}

//let g = &mut gen(task_rng(), DEFAULT_SIZE);

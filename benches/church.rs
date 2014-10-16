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

// local mod imports
use test::{
    prop,
};

// custom mod imports
#[path="../tests/church.rs"]
mod test;

// FIXME: not a good bench; probably too allocation heavy; better than nothing
#[bench]
fn bench(b:&mut std_test::Bencher) -> () {
    let task = || {
        qchk::quickcheck(prop::bool_and_tt);
        qchk::quickcheck(prop::bool_or_ff);
        qchk::quickcheck(prop::bool_and_asc);
        qchk::quickcheck(prop::bool_or_asc);
        qchk::quickcheck(prop::bool_and_comm);
        qchk::quickcheck(prop::bool_or_comm);
        qchk::quickcheck(prop::bool_and_or_red);
        qchk::quickcheck(prop::bool_or_and_red);
        qchk::quickcheck(prop::bool_and_or_dist);
        qchk::quickcheck(prop::bool_or_and_dist);
        qchk::quickcheck(prop::bool_law_non_contradiction);
        qchk::quickcheck(prop::bool_law_excluded_middle)
    };
    b.iter(task);
}

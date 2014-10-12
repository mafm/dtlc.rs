use core::domain::{
    nrm,
};
use core::syntax::{
    chk,
    inf,
};

mod dom {
    use core::domain::{
        neu,
        nrm,
    };
    use core::syntax::{
        sym,
    };
    use core::normal::{
        chk,
    };

    pub fn par(n:sym::Sym) -> nrm::Nrm {
        nrm::Neu(box neu::Par(n))
    }

    pub fn app(v1:nrm::Nrm, v2:nrm::Nrm) -> nrm::Nrm {
        match v1 {
            nrm::Abs(box v1c, mut v1e) => { v1e.push(v2); chk(v1c, v1e) },
            nrm::Neu(v1n) => { nrm::Neu(box neu::App(v1n, box v2)) },
        }
    }
}

pub fn chk(c:chk::Chk, e:nrm::Env) -> nrm::Nrm {
    match c {
        chk::Inf(box ci) => { inf(ci, e) },
        chk::Abs(cc) => { nrm::Abs(cc, e) },
    }
}

fn inf(i:inf::Inf, e:nrm::Env) -> nrm::Nrm {
    match i {
        inf::Ann(box ic, _) => { chk(ic, e) },
        inf::App(box ii, box ic) => { dom::app(inf(ii, e.clone()), chk(ic, e)) },
        inf::Par(ix) => { dom::par(ix) },
        inf::Var(iu) => { e[iu].clone() },
    }
}

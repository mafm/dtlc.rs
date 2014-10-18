use super::domain::{
    nrm,
};
use super::syntax::{
    chk,
    inf,
};

mod sem {
    use super::super::domain::{
        neu,
        nrm,
    };
    use super::super::syntax::{
        sym,
    };
    use super::{
        chk,
    };

    pub fn par(n:sym::Sym) -> nrm::Nrm {
        // println!("par :: n={}", n)
        nrm::Neu(box neu::Par(n))
    }

    pub fn app(v1:nrm::Nrm, v2:nrm::Nrm) -> nrm::Nrm {
        match v1 {
            nrm::Abs(v1c, mut v1e) => {
                // println!("app :: Abs  |\n\t\tv1c = {}\n\t\tv1e = {}\n\t\t v2 = {}", v1c, v1e, v2);
                v1e.push(v2); chk(v1c, v1e)
            },
            nrm::Neu(v1n) => {
                // println!("app :: Neu  |\n\t\tv1n = {}\n\t\t v2 = {}", v1n, v2);
                nrm::Neu(box neu::App(v1n, v2))
            },
        }
    }
}

pub fn lookup(i:uint, e:nrm::Env) -> nrm::Nrm {
    // index relative to the end since Vec::push adds elements at the back
    e[ e.len() - (i + 1) ].clone()
}

pub fn chk(c:chk::Chk, e:nrm::Env) -> nrm::Nrm {
    match c {
        chk::Inf(box ci) => {
            // println!("chk :: Inf  |\n\t\t  e = {}\n\t\t ci = {}", e, ci);
            inf(ci, e)
        },
        chk::Abs(box cc) => {
            // println!("chk :: Abs  |\n\t\t  e = {}\n\t\t cc = {}", e, cc);
            nrm::Abs(cc, e)
        },
    }
}

fn inf(i:inf::Inf, e:nrm::Env) -> nrm::Nrm {
    match i {
        inf::Ann(ic, _) => {
            // println!("inf :: Ann  |\n\t\t  e = {}\n\t\t ic = {}", e, ic);
            chk(ic, e)
        },
        inf::App(box ii, ic) => {
            // println!("inf :: App  |\n\t\t  e = {}\n\t\t ii = {}\n\t\t ic = {}", e, ii, ic);
            sem::app(inf(ii, e.clone()), chk(ic, e))
        },
        inf::Par(ix) => {
            // println!("inf :: Par  |\n\t\t  e = {}\n\t\t ix = {}", e, ix);
            sem::par(ix)
        },
        inf::Var(iu) => {
            // println!("inf :: Var  |\n\t\t  e = {}\n\t\t iu = {}", e, iu);
            lookup(iu, e)
        },
    }
}

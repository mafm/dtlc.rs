use core::model;
use core::syntax;

pub type Env = Vec<model::Value>;

fn vpar(n:syntax::Name,) -> model::Value {
    // println!("vpar");
    model::VNeutral(
        box model::NPar(n)
    )
}

fn vapp(v1:model::Value, v2:model::Value) -> model::Value {
    match v1 {
        model::VLam(box v1c, mut v1e) => {
            // println!("vapp, vlam, v1f=<{}>, v2=<{}>", v1f.clone(), v2.clone());
            v1e.push(v2);
            chk(v1c, v1e)
        },
        model::VNeutral(v1n) => {
            // println!("vapp, vneutral, v1n=<{}>, v2=<{}>", v1n.clone(), v2.clone());
            model::VNeutral(
                box model::NApp(v1n, box v2)
            )
        },
    }
}

pub fn chk(c:syntax::CTerm, e:Env) -> model::Value {
    match c {
        syntax::Inf(box ci) => {
            // println!("chk, inf, e=<{}>, ci=<{}>", e, ci.clone());
            inf(ci, e)
        },
        syntax::Lam(cc) => {
            // println!("chk, lam, e=<{}>, cc=<{}>", e, cc.clone());
            model::VLam(cc, e)
        },
    }
}

fn inf(i:syntax::ITerm, e:Env) -> model::Value {
    match i {
        syntax::Ann(box ic, _) => {
            // println!("inf, ann, e=<{}>, ic=<{}>, t=<{}>", e, ic.clone(), t.clone());
            chk(ic, e)
        },
        syntax::App(box ii, box ic) => {
            // println!("inf, app, e=<{}>, ii=<{}>, ic=<{}>", e, ii.clone(), ic.clone());
            vapp(inf(ii, e.clone()), chk(ic, e))
        },
        syntax::Par(ix) => {
            // println!("inf, par, e=<{}>, ix=<{}>", e, ix.clone());
            vpar(ix)
        },
        syntax::Var(iu) => {
            // println!("inf, var, e=<{}>, iu=<{}>", e, iu.clone());
            e[iu].clone()
        },
    }
}

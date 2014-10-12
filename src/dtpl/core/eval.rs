use core::model;
use core::syntax;

pub type Env = Vec<Box<model::Value>>;

fn vpar(n:Box<syntax::Name>,) -> Box<model::Value> {
    // println!("vpar");
    box model::VNeutral(
        box model::NPar(n)
    )
}

fn vapp(v1:Box<model::Value>, v2:Box<model::Value>) -> Box<model::Value> {
    match v1 {
        box model::VLam(mut v1f) => {
            // println!("vapp, vlam, v1f=<{}>, v2=<{}>", v1f.clone(), v2.clone());
            v1f.apply(v2)
        },
        box model::VNeutral(v1n) => {
            // println!("vapp, vneutral, v1n=<{}>, v2=<{}>", v1n.clone(), v2.clone());
            box model::VNeutral(
                box model::NApp(v1n, v2)
            )
        },
    }
}

pub fn chk(c:Box<syntax::CTerm>, e:Env) -> Box<model::Value> {
    match c {
        box syntax::Inf(ci) => {
            // println!("chk, inf, e=<{}>, ci=<{}>", e, ci.clone());
            inf(ci, e)
        },
        box syntax::Lam(cc) => {
            // println!("chk, lam, e=<{}>, cc=<{}>", e, cc.clone());
            box model::VLam(
                box model::FunLike::new(cc, e)
            )
        },
    }
}

fn inf(i:Box<syntax::ITerm>, e:Env) -> Box<model::Value> {
    match i {
        box syntax::Ann(ref ic, ref t) => {
            // println!("inf, ann, e=<{}>, ic=<{}>, t=<{}>", e, ic.clone(), t.clone());
            chk(ic.clone(), e)
        },
        box syntax::App(ref ii, ref ic) => {
            // println!("inf, app, e=<{}>, ii=<{}>, ic=<{}>", e, ii.clone(), ic.clone());
            vapp(inf(ii.clone(), e.clone()), chk(ic.clone(), e))
        },
        box syntax::Par(ix) => {
            // println!("inf, par, e=<{}>, ix=<{}>", e, ix.clone());
            vpar(ix)
        },
        box syntax::Var(iu) => {
            // println!("inf, var, e=<{}>, iu=<{}>", e, iu.clone());
            e[iu].clone()
        },
    }
}

use core::model;
use core::syntax;

type Env = Vec<Box<model::Value>>;

fn vpar(n:Box<syntax::Name>,) -> Box<model::Value> {
    box model::VNeutral(
        box model::NPar(n)
    )
}

fn vapp(v1:Box<model::Value>, v2:Box<model::Value>,) -> Box<model::Value> {
    match v1 {
        // box model::VLam(mut v1f) => {
        //     v1f(v2)
        // },
        box model::VNeutral(v1n) => {
            box model::VNeutral(
                box model::NApp(v1n, v2)
            )
        },
    }
}

fn ceval(c:Box<syntax::CTerm>, mut e:Env) -> Box<model::Value> {
    match c {
        box syntax::Inf(ci) => {
            ieval(ci, e)
        },
        box syntax::Lam(cc) => {
            // FIXME:
            box model::VNeutral(
                box model::NPar(
                    box syntax::Bound(42)
                )
            )
        },
    }
}

fn ieval(i:Box<syntax::ITerm>, e:Env) -> Box<model::Value> {
    match i {
        box syntax::Ann(ic, _) => {
            ceval(ic, e)
        },
        box syntax::App(ref ii, ref ic) => {
            vapp(ieval(ii.clone(), e.clone()), ceval(ic.clone(), e))
        },
        box syntax::Par(ix) => {
            vpar(ix)
        },
        box syntax::Var(iu) => {
            // FIXME:
            box model::VNeutral(
                box model::NPar(
                    box syntax::Bound(42)
                )
            )
        },
    }
}

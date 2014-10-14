extern crate dtpl;

use dtpl::core::normal;
use dtpl::core::domain::{
    nrm,
};
use dtpl::core::syntax::{
    chk,
    inf,
    sym,
    typ,
};
use dtpl::parse;

fn normal() {
    println!("normalizing:\n\t\"((\\x. x) : A -> A) welp\"");
    let term_orig: chk::Chk =
        chk::Inf(
            box inf::App(
                box inf::Ann(
                    box chk::Abs(
                        box chk::Inf(
                            box inf::Var(0)
                        )
                    ),
                    box typ::Fun(
                        box typ::Par(sym::Con(String::from_str("A"))),
                        box typ::Par(sym::Con(String::from_str("A"))),
                    ),
                ),
                box chk::Inf(
                    box inf::Par(sym::Con(String::from_str("welp")))
                ),
            )
        );
        let term_normal: nrm::Nrm = normal::chk(term_orig, vec![]);
        println!("result:\n\t{}", term_normal);
    }

fn parse() {
    println!("parsing:\n\t\"42\"")
    let sym: Box<sym::Sym> = box sym::Con(String::from_str("42"));
    let res: Result<Box<sym::Sym>,String> = parse::sym("42");
    assert_eq!(res, Ok(sym));
    println!("parsed:\n\t{}", res);
}

fn render() {
    let ty_par: Box<typ::Typ> = box typ::Par(sym::Con(String::from_str("A")));
    let ty_fun: Box<typ::Typ> = box typ::Fun(ty_par.clone(), ty_par);
    let tm_chk: Box<chk::Chk> = box chk::Abs(box chk::Inf(box inf::Var(0)));
    println!("printing:\n\t\"A -> A\":\n\t\t{}", ty_fun);
    println!("printing:\n\t\"\\x. x\":\n\t\t{}", tm_chk);
}

#[test]
fn test () {
    normal();
    println!("\n---\n");
    parse();
    println!("\n---\n");
    render();
}

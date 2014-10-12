extern crate dtpl;

use dtpl::core::syntax;
use dtpl::core::model;
use dtpl::core::eval;
use dtpl::parse;

fn example_eval() {
    println!("evaluating:\n\t\"((\\x. x) : A -> A) welp\"");
    let term_orig: Box<syntax::CTerm> =
        box syntax::Inf(
            box syntax::App(
                box syntax::Ann(
                    box syntax::Lam(
                        box syntax::Inf(
                            box syntax::Var(0)
                        )
                    ),
                    box syntax::Fun(
                        box syntax::TPar(
                            box syntax::Const(String::from_str("uint"))
                        ),
                        box syntax::TPar(
                            box syntax::Const(String::from_str("uint"))
                        ),
                    ),
                ),
                box syntax::Inf(
                    box syntax::Par(
                        box syntax::Const(String::from_str("welp"))
                    )
                ),
            )
        );
    let term_eval: Box<model::Value> = eval::ceval(term_orig, Vec::new());
    println!("result:\n\t{}", term_eval);
}

fn example_parse() {
    println!("parsing:\n\t\"42\"")
    let name: Box<syntax::Name> = box syntax::Bound(42);
    let pres: Result<Box<syntax::Name>,String> = parse::name("42");
    assert_eq!(pres, Ok(name));
    println!("parsed:\n\t{}", pres);
}

fn example_print() {
    // A
    let ty_atom: Box<syntax::Type> =
        box syntax::TPar(
            box syntax::Const(
                String::from_str("A")
            )
        );
    // A -> A
    let ty_fun: Box<syntax::Type> =
        box syntax::Fun(
            ty_atom.clone(),
            ty_atom
        );
    // \x. x
    let term: Box<syntax::CTerm> =
        box syntax::Lam(
            box syntax::Inf(
                box syntax::Var(0)
            )
        );
    println!("printing:\n\t\"A -> A\":\n\t\t{}", ty_fun);
    println!("printing:\n\t\"\\x. x\":\n\t\t{}", term);
}

fn main() {
    example_eval();
    println!("\n---\n");
    example_parse();
    println!("\n---\n");
    example_print();
}

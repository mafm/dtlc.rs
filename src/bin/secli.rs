extern crate selib;

use selib::parse;
use selib::core::syntax;

fn example_parse() {
    println!("parsing :\n\t\"42\"", )
    let name: Box<syntax::Name> = box syntax::Bound(42);
    let pres: Result<Box<syntax::Name>,String> = parse::name("42");
    assert_eq!(pres, Ok(name));
    println!("parsed  :\n\t{}", pres);
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
    println!("printing \"A -> A\" :\n\t{}", ty_fun);
    println!("printing \"\\x. x\"  :\n\t{}", term);
}

fn main() {
    example_parse();
    println!("\n---\n");
    example_print();
}

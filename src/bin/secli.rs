extern crate selib;

use selib::core::syntax;

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
    example_print();
}

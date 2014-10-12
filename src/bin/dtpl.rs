extern crate dtpl;

mod example {
    use dtpl::core::eval;
    use dtpl::core::model::{
        nrm,
    };
    use dtpl::core::syntax::{
        chk,
        inf,
        sym,
        typ,
    };
    use dtpl::parse;

    pub fn eval() {
        println!("evaluating:\n\t\"((\\x. x) : A -> A) welp\"");
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
            let term_eval: nrm::Nrm = eval::chk(term_orig, Vec::new());
            println!("result:\n\t{}", term_eval);
        }

    pub fn parse() {
        println!("parsing:\n\t\"42\"")
        let sym: Box<sym::Sym> = box sym::Con(String::from_str("42"));
        let res: Result<Box<sym::Sym>,String> = parse::sym("42");
        assert_eq!(res, Ok(sym));
        println!("parsed:\n\t{}", res);
    }

    pub fn render() {
        let ty_par: Box<typ::Typ> = box typ::Par(sym::Con(String::from_str("A")));
        let ty_fun: Box<typ::Typ> = box typ::Fun(ty_par.clone(), ty_par);
        let tm_chk: Box<chk::Chk> = box chk::Abs(box chk::Inf(box inf::Var(0)));
        println!("printing:\n\t\"A -> A\":\n\t\t{}", ty_fun);
        println!("printing:\n\t\"\\x. x\":\n\t\t{}", tm_chk);
    }
}

fn main() {
    example::eval();
    println!("\n---\n");
    example::parse();
    println!("\n---\n");
    example::render();
}

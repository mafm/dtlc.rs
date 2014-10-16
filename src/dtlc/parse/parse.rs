#[phase(plugin)]
extern crate peg_syntax_ext;

pub use self::peg::sym;

peg! peg(r#"
use core::syntax;

#[pub]
sym
  -> Box<syntax::sym::Sym>
    = [0-9]+
        {
            box syntax::sym::Con(String::from_str(match_str))
        }
"#)

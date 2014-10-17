#[phase(plugin)]
extern crate peg_syntax_ext;

pub use self::peg::sym;

peg! peg(r#"
use core::lang::stlc::syntax;

#[pub]
sym
  -> Box<syntax::sym::Sym>
    = [0-9]+
        {
            box syntax::sym::Con(String::from_str(match_str))
        }
"#)

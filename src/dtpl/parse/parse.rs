#[phase(plugin)]
extern crate peg_syntax_ext;

pub use self::peg::name;

peg! peg(r#"
use core::syntax;

#[pub]
name
  -> Box<syntax::Name>
    = [0-9]+
        {
            box syntax::Const(
                String::from_str(match_str)
            )
        }
"#)

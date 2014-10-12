use core::eval;
use core::syntax;

#[deriving(Clone)]
#[deriving(Show)]
pub enum Neutral {
    NPar(syntax::Name),
    NApp(Box<Neutral>, Box<Value>),
}

#[deriving(Clone)]
#[deriving(Show)]
pub enum Value {
    VLam(Box<syntax::CTerm>, eval::Env),
    VNeutral(Box<Neutral>),
}

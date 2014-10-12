use core::syntax;

#[deriving(Clone)]
pub enum Neutral {
    NPar(Box<syntax::Name>),
    NApp(Box<Neutral>, Box<Value>),
}

#[deriving(Clone)]
pub enum Value {
    // FIXME: will probably need a struct/object for VLam
    // VLam,
    VNeutral(Box<Neutral>),
}

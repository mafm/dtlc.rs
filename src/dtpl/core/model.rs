use core::syntax;

pub enum Neutral {
    NPar(Box<syntax::Name>),
    NApp(Box<Neutral>, Box<Value>),
}

pub enum Value {
    VLam(Box<|&mut: Box<Value>| -> Box<Value>>),
    VNeutral(Box<Neutral>),
}

use core::eval;
use core::syntax;

#[deriving(Clone)]
#[deriving(Show)]
pub enum Neutral {
    NPar(Box<syntax::Name>),
    NApp(Box<Neutral>, Box<Value>),
}

#[deriving(Clone)]
#[deriving(Show)]
pub enum Value {
    VLam(Box<Closure>),
    VNeutral(Box<Neutral>),
}

#[deriving(Clone)]
#[deriving(Show)]
pub struct Closure {
    chk: Box<syntax::CTerm>,
    env: Vec<Box<Value>>,
}

// FIXME:
//   We can probably generalize this by using tuple args for the context and
//   passing the bare functions (or maybe closures) consuming the context to
//   the apply method.
//
//   Another possibility might be to use traits to implement different versions
//   of the apply method. We might need to make Closure parametric somehow in
//   that case.
//
//   Also might be able to do something clever with associated types and
//   multidispatch to get a nicer encoding.
impl Closure {
    pub fn new(c:Box<syntax::CTerm>, e:Vec<Box<Value>>) -> Closure {
        // println!("new")
        Closure {
            chk: c,
            env: e,
        }
    }
    pub fn apply(&mut self, v:Box<Value>) -> Box<Value> {
        // println!("applying")
        self.env.push(v);
        eval::chk(self.chk.clone(), self.env.clone())
    }
}

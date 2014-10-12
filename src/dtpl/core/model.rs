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
    VLam(Box<FunLike>),
    VNeutral(Box<Neutral>),
}

#[deriving(Clone)]
#[deriving(Show)]
pub struct FunLike {
    chk: Box<syntax::CTerm>,
    env: Vec<Box<Value>>,
}

impl FunLike {
    pub fn new(c:Box<syntax::CTerm>, e:Vec<Box<Value>>) -> FunLike {
        // println!("new")
        FunLike {
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

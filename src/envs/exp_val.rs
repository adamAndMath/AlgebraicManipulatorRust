use env::ID;
use id::{ Type, Exp, ErrID, SetLocal };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpVal {
    val: Option<Exp>,
    gen: usize,
    ty: Type,
}

impl ExpVal {
    pub fn new_empty(ty: Type, gen: usize) -> Self {
        ExpVal { val: None, ty, gen }
    }

    pub fn new(e: Exp, ty: Type, gen: usize) -> Self {
        ExpVal { val: Some(e), ty, gen }
    }

    pub fn set_val(&mut self, e: Exp) {
        if self.val != None {
            panic!("Value is set twice");
        }

        self.val = Some(e);
    }

    pub fn val(&self, id: ID<Self>, gen: &[Type]) -> Result<Exp, ErrID> {
        self.val.as_ref().map(|e|id.move_into(e).set(gen)).ok_or(ErrID::VarNotSet(id))
    }

    pub fn ty(&self, id: ID<Self>, gen: &[Type]) -> Type {
        id.move_into(&self.ty).set(gen)
    }
}

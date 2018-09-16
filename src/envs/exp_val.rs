use id::{ Type, Exp };

#[derive(Debug, Clone, PartialEq)]
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

    pub fn val(&self) -> Option<Exp> {
        self.val.clone()
    }

    pub fn ty(&self) -> Type {
        self.ty.clone()
    }
}

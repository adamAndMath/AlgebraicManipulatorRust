pub trait PushID: Sized {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self;

    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self>;

    fn push_id(&self, amount: usize) -> Self {
        self.push_id_with_min(0, amount)
    }

    fn pop_id(&self, amount: usize) -> Option<Self> {
        self.pop_id_with_min(0, amount)
    }
}

impl<T: PushID> PushID for Box<T> {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        Box::new((&**self).push_id_with_min(min, amount))
    }
    
    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        (&**self).pop_id_with_min(min, amount).map(Box::new)
    }
}

impl<T: PushID> PushID for Vec<T> {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        self.into_iter().map(|e|e.push_id_with_min(min, amount)).collect()
    }
    
    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        self.into_iter().map(|e|e.pop_id_with_min(min, amount)).collect()
    }
}

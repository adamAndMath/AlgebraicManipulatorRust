use std::marker::PhantomData;
use super::PushID;

#[derive(Debug)]
pub enum ID<T: ?Sized> {
    Predef(usize, PhantomData<T>),
    Normal(usize, usize, PhantomData<T>),
}

impl<T: ?Sized> ID<T> {
    pub fn new(id: usize) -> Self {
        ID::Normal(id, 0, PhantomData)
    }

    pub fn move_into<U: PushID>(&self, e: &U) -> U {
        match self {
            ID::Predef(_, _) => e.push_id(0),
            ID::Normal(_, up, _) => e.push_id(*up),
        }
    }

    pub fn set(&self, min: usize) -> Result<usize, ID<T>> {
        match self {
            ID::Predef(id, p) => Err(ID::Predef(*id, p.clone())),
            ID::Normal(id, up, p) if *up < min => Err(ID::Normal(*id, *up, p.clone())),
            ID::Normal(id, up, p) if *up > min => Err(ID::Normal(*id, up - 1, p.clone())),
            ID::Normal(id, _, _) => Ok(*id),
        }
    }
}

impl<T: ?Sized> PushID for ID<T> {
    fn push_id_with_min(&self, min: usize, amount: usize) -> Self {
        match self {
            ID::Predef(id, p) => ID::Predef(*id, p.clone()),
            ID::Normal(id, up, p) => ID::Normal(*id, if *up >= min { up + amount } else { *up }, p.clone()),
        }
    }

    fn pop_id_with_min(&self, min: usize, amount: usize) -> Option<Self> {
        Some(match self {
            ID::Predef(id, p) => ID::Predef(*id, p.clone()),
            ID::Normal(id, up, p) => ID::Normal(*id, 
                if *up >= min + amount {
                    up - amount
                } else if *up >= min {
                    return None;
                } else {
                    *up
                }
            , p.clone()),
        })
    }
}

impl<T: ?Sized> Clone for ID<T> {
    fn clone(&self) -> Self {
        match self {
            ID::Predef(id, p) => ID::Predef(*id, p.clone()),
            ID::Normal(id, up, p) => ID::Normal(*id, *up, p.clone()),
        }
    }
}

impl<T: ?Sized> PartialEq for ID<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ID::Predef(lhs, _), ID::Predef(rhs, _)) => lhs == rhs,
            (ID::Normal(l_id, l_up, _), ID::Normal(r_id, r_up, _)) => l_id == r_id && l_up == r_up,
            _ => false,
        }
    }
}

impl<T: ?Sized> Eq for ID<T> {}

impl<T: ?Sized> Copy for ID<T> {}

mod ty;
mod exp;
mod pattern;
mod proof;
mod element;
mod id_err;
mod type_check;
mod set_local;

pub use self::ty::Type;
pub use self::exp::Exp;
pub use self::pattern::Pattern;
pub use self::proof::*;
pub use self::element::Element;
pub use self::id_err::ErrID;
pub use self::type_check::TypeCheck;
pub use self::type_check::TypeCheckIter;
pub use self::set_local::SetLocal;

pub mod renamed {
    pub use super::{
        Type as TypeID,
        Exp as ExpID,
        Pattern as PatternID,
        TruthRef as TruthRefID,
        Proof as ProofID,
        Element as ElementID,
        Direction,
        RefType,
        ErrID,
        TypeCheck,
        SetLocal
    };
}
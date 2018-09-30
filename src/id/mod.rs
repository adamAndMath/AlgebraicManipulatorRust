mod ty;
mod exp;
mod pattern;
mod proof;
mod id_err;
mod type_check;
mod set_local;

pub use self::ty::Type;
pub use self::exp::Exp;
pub use self::pattern::Pattern;
pub use self::id_err::ErrID;
pub use self::proof::*;
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
        Direction,
        MatchEnv,
        RefType,
        ErrID,
        TypeCheck,
        SetLocal
    };
}
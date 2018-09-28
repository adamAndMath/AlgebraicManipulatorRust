mod ty;
mod exp;
mod pattern;
mod proof;
mod id_err;

pub use self::ty::Type;
pub use self::exp::Exp;
pub use self::pattern::Pattern;
pub use self::id_err::ErrID;
pub use self::proof::*;

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
    };
}
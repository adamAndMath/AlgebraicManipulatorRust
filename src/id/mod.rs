mod ty;
mod exp;
mod pattern;

pub use self::ty::Type;
pub use self::exp::Exp;
pub use self::pattern::Pattern;

pub mod renamed {
    pub use super::{
        Type as TypeID,
        Exp as ExpID,
        Pattern as PatternID,
    };
}
use std::ops::{ Neg, Not, BitAnd, BitOr };
use self::Variance::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variance {
    Invariant,
    Covariant,
    Contravariant,
    Bivariant,
}

impl Neg for Variance {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Invariant => Invariant,
            Covariant => Contravariant,
            Contravariant => Covariant,
            Bivariant => Bivariant,
        }
    }
}

impl Not for Variance {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Invariant => Bivariant,
            Covariant => Contravariant,
            Contravariant => Covariant,
            Bivariant => Invariant,
        }
    }
}

impl BitAnd for Variance {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Invariant, _) | (_, Invariant) | (Covariant, Contravariant) | (Contravariant, Covariant) => Invariant,
            (Covariant, _) | (_, Covariant) => Covariant,
            (Contravariant, _) | (_, Contravariant) => Contravariant,
            (Bivariant, Bivariant) => Bivariant,
        }
    }
}

impl BitOr for Variance {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Bivariant, _) | (_, Bivariant) | (Covariant, Contravariant) | (Contravariant, Covariant) => Bivariant,
            (Covariant, _) | (_, Covariant) => Covariant,
            (Contravariant, _) | (_, Contravariant) => Contravariant,
            (Invariant, Invariant) => Invariant,
        }
    }
}
use variance::Variance::*;

#[test]
fn neg() {
    assert_eq!(-Invariant, Invariant);
    assert_eq!(-Covariant, Contravariant);
    assert_eq!(-Contravariant, Covariant);
    assert_eq!(-Bivariant, Bivariant);
}

#[test]
fn not() {
    assert_eq!(!Invariant, Bivariant);
    assert_eq!(!Covariant, Contravariant);
    assert_eq!(!Contravariant, Covariant);
    assert_eq!(!Bivariant, Invariant);
}

#[test]
fn and() {
    assert_eq!(Covariant&Covariant, Covariant);
    assert_eq!(Contravariant&Contravariant, Contravariant);
    assert_eq!(Contravariant&Covariant, Invariant);
    assert_eq!(Covariant&Contravariant, Invariant);
}

#[test]
fn or() {
    assert_eq!(Covariant|Covariant, Covariant);
    assert_eq!(Contravariant|Contravariant, Contravariant);
    assert_eq!(Contravariant|Covariant, Bivariant);
    assert_eq!(Covariant|Contravariant, Bivariant);
}

#![feature(range_contains, box_patterns, transpose_result)]

mod env;
mod envs;
mod ast;
mod id;
mod predef;
mod variance;
mod tree;

#[macro_use]
mod macros;

#[cfg(test)]
mod test;

fn main() {
    use predef::*;
    use envs::Envs;
    use variance::Variance;
    use tree::Tree;
    use ast::*;
    use id::Direction;

    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);
    alias_predef(&mut env);

    script! {env,
        fn not {
            true => false,
            false => true
        }

        proof DoubleNegative(b: Bool) {
            match b {
                true => ID(not(not(true)))~wrap()[1,0]~wrap()[1]~match(b)[0,0,0|1],
                false => ID(not(not(false)))~wrap()[1,0]~wrap()[1]~match(b)[0,0,0|1]
            }
        }

        fn and {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => false
        }

        fn or {
            (true, true) => true,
            (true, false) => true,
            (false, true) => true,
            (false, false) => false
        }

        proof And_Commutative(a: Bool, b: Bool) {
            match (a, b) {
                (true, true) => ID(and(true, true))~match(a)[0,0|1,1]~match(b)[0,1|1,0],
                (true, false) => ID(and(true, false))~wrap()[1].wrap(and(false, true))[1]~match(a)[0,0|1,1]~match(b)[0,1|1,0],
                (false, true) => ID(and(false, true))~wrap()[1].wrap(and(true, false))[1]~match(a)[0,0|1,1]~match(b)[0,1|1,0],
                (false, false) => ID(and(false, false))~match(a)[0,0|1,1]~match(b)[0,1|1,0]
            }
        }

        proof And_NeutralElement_Left(b: Bool) {
            match b {
                true => ID(and(true, true))~wrap()[1]~match(b)[0,1|1],
                false => ID(and(true, false))~wrap()[1]~match(b)[0,1|1]
            }
        }

        proof And_NeutralElement_Right(b: Bool) {
            ID(and(b, true)).And_Commutative(b, true)[1].And_NeutralElement_Left(b)[1]
        }

        proof And_AbsorbativeElement_Left(b: Bool) {
            match b {
                true => ID(and(false, b)).match()[1,1]~wrap()[1],
                false => ID(and(false, b)).match()[1,1]~wrap()[1]
            }
        }

        proof And_AbsorbativeElement_Right(b: Bool) {
            ID(and(b, false)).And_Commutative(b, false)[1].And_AbsorbativeElement_Left(b)[1]
        }

        proof And_Associative(a: Bool, b: Bool, c: Bool) {
            match b {
                true => ID(and(and(a, true), c))
                            .And_NeutralElement_Right(a)[1,0]
                            ~And_NeutralElement_Left(c)[1,1],
                false => ID(and(and(a, false), c))
                            .And_AbsorbativeElement_Right(a)[1,0]
                            .And_NeutralElement_Left(c)[1]
                            ~And_AbsorbativeElement_Right(a)[1]
                            ~And_AbsorbativeElement_Left(c)[1,1]
            }
        }

        enum Nat {
            Zero,
            Succ(Nat)
        }

        fn add -> Nat {
            (a: Nat, Zero) => a,
            (a: Nat, Succ(p: Nat)) => Succ(add(a, p))
        }

        fn mul -> Nat {
            (a: Nat, Zero) => Zero,
            (a: Nat, Succ(p: Nat)) => add(mul(a, p), a)
        }

        fn pow -> Nat {
            (a: Nat, Zero) => Succ(Zero),
            (a: Nat, Succ(p: Nat)) => mul(pow(a, p), a)
        }

        enum IntP {
            One,
            Succ(IntP)
        }

        fn add -> IntP {
            (a: IntP, One) => Succ(a),
            (a: IntP, Succ(p: IntP)) => Succ(add(a, p))
        }

        fn mul -> IntP {
            (a: IntP, One) => a,
            (a: IntP, Succ(p: IntP)) => add(mul(a, p), a)
        }

        fn pow -> IntP {
            (a: IntP, One) => a,
            (a: IntP, Succ(p: IntP)) => mul(pow(a, p), a)
        }

        enum Int {
            Zero,
            NoneZero(Bool, IntP)
        }
    }
}
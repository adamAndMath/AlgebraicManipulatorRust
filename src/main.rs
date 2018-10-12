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
    use env::Path;
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
                true => ID[Bool](not(not(true))).def(not(true))[1,0].def(not(false))[1]~match(b)[0,0,0|1],
                false => ID[Bool](not(not(false))).def(not(false))[1,0].def(not(true))[1]~match(b)[0,0,0|1]
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
                (true, true) => ID[Bool](and(true, true))~match(a)[0,0|1,1]~match(b)[0,1|1,0],
                (true, false) => def(and(true, false))~def(and(false, true))[1]~match(a)[0,0|1,1]~match(b)[0,1|1,0],
                (false, true) => def(and(false, true))~def(and(true, false))[1]~match(a)[0,0|1,1]~match(b)[0,1|1,0],
                (false, false) => ID[Bool](and(false, false))~match(a)[0,0|1,1]~match(b)[0,1|1,0]
            }
        }

        proof And_NeutralElement_Left(b: Bool) {
            match b {
                true => def(and(true, true))~match(b)[0,1|1],
                false => def(and(true, false))~match(b)[0,1|1]
            }
        }

        proof And_NeutralElement_Right(b: Bool) {
            And_Commutative(b, true).And_NeutralElement_Left(b)[1]
        }

        proof And_AbsorbativeElement_Left(b: Bool) {
            match b {
                true => ID[Bool](and(false, b)).match(b)[1,1].def(and(false, true))[1],
                false => ID[Bool](and(false, b)).match(b)[1,1].def(and(false, false))[1]
            }
        }

        proof And_AbsorbativeElement_Right(b: Bool) {
            And_Commutative(b, false).And_AbsorbativeElement_Left(b)[1]
        }

        proof And_Associative(a: Bool, b: Bool, c: Bool) {
            match b {
                true => ID[Bool](and(and(a, true), c))
                            .And_NeutralElement_Right(a)[1,0]
                            ~And_NeutralElement_Left(c)[1,1]
                            ~match(b)[0,0,1|1,1,0],
                false => ID[Bool](and(and(a, false), c))
                            .And_AbsorbativeElement_Right(a)[1,0]
                            .And_AbsorbativeElement_Left(c)[1]
                            ~And_AbsorbativeElement_Right(a)[1]
                            ~And_AbsorbativeElement_Left(c)[1,1]
                            ~match(b)[0,0,1|1,1,0]
            }
        }

        enum Nat {
            Zero,
            Succ(Nat)
        }

        fn add -> Nat {
            (a: Nat, Nat::Zero) => a,
            (a: Nat, Nat::Succ(p: Nat)) => Nat::Succ(add(a, p))
        }

        fn mul -> Nat {
            (a: Nat, Nat::Zero) => Nat::Zero,
            (a: Nat, Nat::Succ(p: Nat)) => add(mul(a, p), a)
        }

        fn pow -> Nat {
            (a: Nat, Nat::Zero) => Nat::Succ(Nat::Zero),
            (a: Nat, Nat::Succ(p: Nat)) => mul(pow(a, p), a)
        }

        enum IntP {
            One,
            Succ(IntP)
        }

        fn add -> IntP {
            (a: IntP, IntP::One) => IntP::Succ(a),
            (a: IntP, IntP::Succ(p: IntP)) => IntP::Succ(add(a, p))
        }

        fn mul -> IntP {
            (a: IntP, IntP::One) => a,
            (a: IntP, IntP::Succ(p: IntP)) => add(mul(a, p), a)
        }

        fn pow -> IntP {
            (a: IntP, IntP::One) => a,
            (a: IntP, IntP::Succ(p: IntP)) => mul(pow(a, p), a)
        }

        enum Int {
            Zero,
            NoneZero(Bool, IntP)
        }
    }
}
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
    use ast::*;

    let (mut exps, mut tys) = predef();
    let mut env = Envs::new(&mut exps, &mut tys);
    alias_predef(&mut env);

    script! {env,
        fn not(b: Bool) = match(b, {
            true => false,
            false => true
        });

        fn and(a: Bool, b: Bool) = match((a, b), {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => false
        });

        fn or(a: Bool, b: Bool) = match((a, b), {
            (true, true) => true,
            (true, false) => true,
            (false, true) => true,
            (false, false) => false
        });

        enum Nat {
            Zero,
            Succ(Nat)
        }

        fn add(a: Nat, b: Nat) -> Nat = match(b, {
            Zero => a,
            Succ(p: Nat) => Succ(add(a, p))
        });

        fn mul(a: Nat, b: Nat) -> Nat = match(b, {
            Zero => Zero,
            Succ(p: Nat) => add(mul(a, p), a)
        });

        fn pow(a: Nat, b: Nat) -> Nat = match(b, {
            Zero => Succ(Zero),
            Succ(p: Nat) => mul(pow(a, p), a)
        });

        enum IntP {
            One,
            Succ(IntP)
        }

        fn add(a: IntP, b: IntP) -> IntP = match(b, {
            One => Succ(a),
            Succ(p: IntP) => Succ(add(a, p))
        });

        fn mul(a: IntP, b: IntP) -> IntP = match(b, {
            One => a,
            Succ(p: IntP) => add(mul(a, p), a)
        });

        fn pow(a: IntP, b: IntP) -> IntP = match(b, {
            One => a,
            Succ(p: IntP) => mul(pow(a, p), a)
        });

        enum Int {
            Zero,
            NoneZero(Bool, IntP)
        }
    }
}
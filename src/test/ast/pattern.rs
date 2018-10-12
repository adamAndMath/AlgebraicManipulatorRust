use parser::Parse;
use predef::*;
use envs::*;
use variance::Variance;
use ast::{ Pattern, ToID };
use id::renamed::{ PatternID, TypeID };

#[test]
fn to_id() {
    let (mut exps, mut tys, mut truths) = predef();
    let mut env = Envs::new(&mut exps, &mut tys, &mut truths);

    let nat_id = env.ty.add("Nat".to_owned(), TypeVal::new(vec![]));
    let zero_id = env.exp.add("Zero".to_owned(), ExpVal::new_empty(type_id!(nat_id), 0));
    env.ty.get_mut(nat_id).unwrap().push_atom(zero_id);
    let succ_id = env.exp.add("Succ".to_owned(), ExpVal::new_empty(type_id!(FN_ID[-nat_id, +nat_id]), 0));
    env.ty.get_mut(nat_id).unwrap().push_comp(succ_id);

    let env = env.local();

    assert_eq!(Pattern::parse("Zero").to_id(&env), Ok(pattern_id!(zero_id)));
    assert_eq!(Pattern::parse("n: Nat").to_id(&env), Ok(pattern_id!(+nat_id)));
    assert_eq!(Pattern::parse("Succ(Zero)").to_id(&env), Ok(pattern_id!(succ_id(zero_id))));
    assert_eq!(Pattern::parse("Succ(n: Nat)").to_id(&env), Ok(pattern_id!(succ_id(+nat_id))));
}

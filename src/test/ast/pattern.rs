use predef::*;
use envs::*;
use variance::Variance;
use ast::{ Type, Pattern };
use id::renamed::{ PatternID, TypeID };

#[test]
fn to_id() {
    let (mut exps, mut tys) = predef();
    let mut env = Envs::new(&mut exps, &mut tys);

    let nat_id = env.ty.add("Nat".to_owned(), TypeVal::new(vec![]));
    let zero_id = env.exp.add("Zero".to_owned(), ExpVal::new_empty(type_id!(nat_id)));
    env.ty.get_mut(nat_id).unwrap().push_atom(zero_id);
    let succ_id = env.exp.add("Succ".to_owned(), ExpVal::new_empty(type_id!(FN_ID[-nat_id, +nat_id])));
    env.ty.get_mut(nat_id).unwrap().push_comp(succ_id);

    let env = env.local();

    assert_eq!(pattern!(Zero).to_id(&env), Some(pattern_id!(zero_id)));
    assert_eq!(pattern!(n: Nat).to_id(&env), Some(pattern_id!(+nat_id)));
    assert_eq!(pattern!(Succ(Zero)).to_id(&env), Some(pattern_id!(succ_id(zero_id))));
    assert_eq!(pattern!(Succ(n: Nat)).to_id(&env), Some(pattern_id!(succ_id(+nat_id))));
}
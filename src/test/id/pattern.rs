use predef::*;
use envs::*;
use variance::Variance;
use id::renamed::{ PatternID, TypeID, TypeCheck };

#[test]
fn type_check() {
    let mut data = predef();
    let mut env = Envs::new(&mut data);

    let nat_id = env.ty.add("Nat".to_owned(), TypeVal::new(vec![]));
    let zero_id = env.exp.add("Zero".to_owned(), ExpVal::new_empty(type_id!(nat_id), 0));
    env.ty.get_mut(nat_id).unwrap().push_atom(zero_id);
    let succ_id = env.exp.add("Succ".to_owned(), ExpVal::new_empty(type_id!(FN_ID[-nat_id, +nat_id]), 0));
    env.ty.get_mut(nat_id).unwrap().push_comp(succ_id);

    let env = env.local();
    
    assert_eq!(pattern_id!(zero_id).type_check(&env), Ok(type_id!(nat_id)));
    assert_eq!(pattern_id!(+nat_id).type_check(&env), Ok(type_id!(nat_id)));
    assert_eq!(pattern_id!(succ_id(zero_id)).type_check(&env), Ok(type_id!(nat_id)));
    assert_eq!(pattern_id!(succ_id(+nat_id)).type_check(&env), Ok(type_id!(nat_id)));
}
use predef::*;
use envs::*;
use id::renamed::{ PatternID, TypeID, TypeCheck };

#[test]
fn type_check() {
    let mut env = predef();

    let nat_id = env.ty.add(TypeVal::new(vec![]));
    let zero_id = env.exp.add(ExpVal::new_empty(type_id!(nat_id), 0));
    env.ty[nat_id].push_atom(zero_id);
    let succ_id = env.exp.add(ExpVal::new_empty(type_id!(FN_ID[nat_id, nat_id]), 0));
    env.ty[nat_id].push_comp(succ_id);

    let env = env.local();
    
    assert_eq!(pattern_id!(zero_id).type_check(&env), Ok(type_id!(nat_id)));
    assert_eq!(pattern_id!(+nat_id).type_check(&env), Ok(type_id!(nat_id)));
    assert_eq!(pattern_id!(succ_id(zero_id)).type_check(&env), Ok(type_id!(nat_id)));
    assert_eq!(pattern_id!(succ_id(+nat_id)).type_check(&env), Ok(type_id!(nat_id)));
}
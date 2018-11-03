use predef::*;
use envs::*;
use id::renamed::{ TypeID, ExpID, TypeCheck };

#[test]
fn type_check() {
    let mut env = predef();

    let nat_id = env.ty.add(TypeVal::new(vec!()));
    let zero_id = env.exp.add(ExpVal::new_empty(type_id!(nat_id), 0));
    env.ty[nat_id].push_atom(zero_id);
    let succ_id = env.exp.add(ExpVal::new_empty(type_id!(FN_ID[nat_id, nat_id]), 0));
    env.ty[nat_id].push_comp(succ_id);

    let env = env.local();

    let exp = exp_id!(succ_id(zero_id));

    assert_eq!(exp.type_check(&env), Ok(type_id!(nat_id)));
}
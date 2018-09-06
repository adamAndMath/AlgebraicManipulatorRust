use envs::*;
use ty::*;

#[test]
fn to_id() {
    let mut exps = vec![];
    let mut tys = vec![];
    let mut env = Envs::new(&mut exps, &mut tys);
    
    let a_id = env.ty.add("A".to_owned(), (vec!(), vec!(), vec!()));
    let b_id = env.ty.add("B".to_owned(), (vec!(), vec!(), vec!()));
    let c_id = env.ty.add("C".to_owned(), (vec!(), vec!(), vec!()));

    let env = env.local();

    assert_eq!(ttype!(A).to_id(&env), Some(type_id!(a_id)));
    assert_eq!(ttype!(B).to_id(&env), Some(type_id!(b_id)));
    assert_eq!(ttype!(C).to_id(&env), Some(type_id!(c_id)));
    assert_eq!(ttype!((A, B, C)).to_id(&env), Some(type_id!((a_id, b_id, c_id))));
}

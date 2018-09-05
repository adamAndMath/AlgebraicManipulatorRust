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

    let a = Type::Gen("A".to_owned(), vec!());
    let b = Type::Gen("B".to_owned(), vec!());
    let c = Type::Gen("C".to_owned(), vec!());

    let a_ty = TypeID::Gen(a_id, vec!());
    let b_ty = TypeID::Gen(b_id, vec!());
    let c_ty = TypeID::Gen(c_id, vec!());

    assert_eq!(a.to_id(&env), Some(a_ty.clone()));
    assert_eq!(b.to_id(&env), Some(b_ty.clone()));
    assert_eq!(c.to_id(&env), Some(c_ty.clone()));
    assert_eq!(Type::Tuple(vec!(a, b, c)).to_id(&env), Some(TypeID::Tuple(vec!(a_ty, b_ty, c_ty))));
}

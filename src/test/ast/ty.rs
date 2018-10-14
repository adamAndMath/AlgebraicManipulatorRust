use predef::*;
use env::Path;
use envs::*;
use id::renamed::TypeID;
use ast::{ Type, ToID };

#[test]
fn to_id() {
    let mut data = predef();
    let mut env = Envs::new(&mut data);
    
    let a_id = env.ty.add("A".to_owned(), TypeVal::new(vec!()));
    let b_id = env.ty.add("B".to_owned(), TypeVal::new(vec!()));
    let c_id = env.ty.add("C".to_owned(), TypeVal::new(vec!()));

    let env = env.local();

    assert_eq!(ttype!(A).to_id(&env), Ok(type_id!(a_id)));
    assert_eq!(ttype!(B).to_id(&env), Ok(type_id!(b_id)));
    assert_eq!(ttype!(C).to_id(&env), Ok(type_id!(c_id)));
    assert_eq!(ttype!((A, B, C)).to_id(&env), Ok(type_id!((a_id, b_id, c_id))));
}

use parser::Parse;
use predef::*;
use envs::*;
use id::renamed::TypeID;
use ast::{ Type, ToID };

#[test]
fn to_id() {
    let mut data = predef();
    let mut env = Envs::new("".to_owned(), &mut data);
    
    let a_id = env.ty.add("A", TypeVal::new(vec!()));
    let b_id = env.ty.add("B", TypeVal::new(vec!()));
    let c_id = env.ty.add("C", TypeVal::new(vec!()));

    let env = env.local();

    assert_eq!(Type::parse("A").to_id(&env), Ok(type_id!(a_id)));
    assert_eq!(Type::parse("B").to_id(&env), Ok(type_id!(b_id)));
    assert_eq!(Type::parse("C").to_id(&env), Ok(type_id!(c_id)));
    assert_eq!(Type::parse("(A, B, C)").to_id(&env), Ok(type_id!((a_id, b_id, c_id))));
}

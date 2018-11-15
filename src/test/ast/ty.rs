use parser::Parse;
use predef::*;
use envs::NameData;
use id::renamed::TypeID;
use ast::{ Type, ToID };

#[test]
fn to_id() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);
    
    let a_id = space.types.add(&"A");
    let b_id = space.types.add(&"B");
    let c_id = space.types.add(&"C");

    assert_eq!(Type::parse("A").to_id(&space), Ok(type_id!(a_id)));
    assert_eq!(Type::parse("B").to_id(&space), Ok(type_id!(b_id)));
    assert_eq!(Type::parse("C").to_id(&space), Ok(type_id!(c_id)));
    assert_eq!(Type::parse("(A, B, C)").to_id(&space), Ok(type_id!((a_id, b_id, c_id))));
}

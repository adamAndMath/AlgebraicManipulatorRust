use predef::*;
use id::renamed::TypeID;
use ast::{ Type, ToID };

#[test]
fn to_id() {
    let mut space = predef_space();
    
    let a_id = space.types.add("A".to_owned());
    let b_id = space.types.add("B".to_owned());
    let c_id = space.types.add("C".to_owned());

    let space = &space.local();

    assert_eq!(ttype!(A).to_id(space), Ok(type_id!(a_id)));
    assert_eq!(ttype!(B).to_id(space), Ok(type_id!(b_id)));
    assert_eq!(ttype!(C).to_id(space), Ok(type_id!(c_id)));
    assert_eq!(ttype!((A, B, C)).to_id(space), Ok(type_id!((a_id, b_id, c_id))));
}

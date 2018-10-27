use predef::*;
use ast::{ Type, Pattern, ToID };
use id::renamed::{ PatternID, TypeID };

#[test]
fn to_id() {
    let mut space = predef_space();

    let nat_id = space.types.add("Nat".to_owned());
    let zero_id = space.exps.add("Zero".to_owned());
    let succ_id = space.exps.add("Succ".to_owned());

    let space = &space.local();

    assert_eq!(pattern!(Zero).to_id(space), Ok(pattern_id!(zero_id)));
    assert_eq!(pattern!(n: Nat).to_id(space), Ok(pattern_id!(+nat_id)));
    assert_eq!(pattern!(Succ(Zero)).to_id(space), Ok(pattern_id!(succ_id(zero_id))));
    assert_eq!(pattern!(Succ(n: Nat)).to_id(space), Ok(pattern_id!(succ_id(+nat_id))));
}
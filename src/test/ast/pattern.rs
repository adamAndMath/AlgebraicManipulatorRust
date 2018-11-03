use parser::Parse;
use predef::*;
use envs::NameData;
use ast::{ Pattern, ToID };
use id::renamed::{ PatternID, TypeID };

#[test]
fn to_id() {
    let mut names = NameData::new();
    let mut space = predef_space(&mut names);

    let nat_id = space.types.add(&"Nat");
    let zero_id = space.exps.add(&"Zero");
    let succ_id = space.exps.add(&"Succ");

    let space = &space.local();

    assert_eq!(Pattern::parse("Zero").to_id(space), Ok(pattern_id!(zero_id)));
    assert_eq!(Pattern::parse("n: Nat").to_id(space), Ok(pattern_id!(+nat_id)));
    assert_eq!(Pattern::parse("Succ(Zero)").to_id(space), Ok(pattern_id!(succ_id(zero_id))));
    assert_eq!(Pattern::parse("Succ(n: Nat)").to_id(space), Ok(pattern_id!(succ_id(+nat_id))));
}

use tree::Tree;

#[test]
fn liniar_path() {
    assert_eq!(format!("{}", tree!(1,2,3)), "1,2,3");
}

#[test]
fn branching_path() {
    assert_eq!(format!("{}", tree!([1|2|3])), "[1|2|3]");
}

#[test]
fn branching_with_depth_path() {
    assert_eq!(format!("{}", tree!([1,3|2,[1|2]|3])), "[1,3|2,[1|2]|3]");
}

#[test]
fn compact_path() {
    assert_eq!(format!("{}", tree!([0|1,0],[1|0,1])), "[0,[0,1|1]|1,0,[0,1|1]]");
}

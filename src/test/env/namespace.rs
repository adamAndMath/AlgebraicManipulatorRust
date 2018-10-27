use std::collections::HashMap;
use std::marker::PhantomData;
use env::{ ID, Namespace };

#[test]
#[allow(unused_variables)]
fn create_empty_space() {
    let space: Namespace<&str> = Namespace::new(HashMap::new());
}

#[test]
fn create_space_with_a_b_c() {
    let space: Namespace<&str> = Namespace::new(vec![
        ("a".to_owned(), ID::Predef(0, PhantomData)),
        ("b".to_owned(), ID::Predef(1, PhantomData)),
        ("c".to_owned(), ID::Predef(2, PhantomData)),
    ]);

    assert_eq!(space.get(&path![a]), Ok(ID::Predef(0, PhantomData)));
    assert_eq!(space.get(&path![b]), Ok(ID::Predef(1, PhantomData)));
    assert_eq!(space.get(&path![c]), Ok(ID::Predef(2, PhantomData)));
}

#[test]
fn add_data_to_empty_space() {
    let mut space: Namespace<&str> = Namespace::new(HashMap::new());

    space.add("a".to_owned());
    space.add("b".to_owned());
    space.add("c".to_owned());

    assert_eq!(space.get(&path!(a)), Ok(ID::new(0)));
    assert_eq!(space.get(&path!(b)), Ok(ID::new(1)));
    assert_eq!(space.get(&path!(c)), Ok(ID::new(2)));

    assert_eq!(space.paths(), [path![a], path![b], path![c]]);
}

#[test]
fn add_data_in_and_after_scope() {
    let mut space: Namespace<&str> = Namespace::new(vec![("Predef".to_owned(), ID::Predef(0, PhantomData))]);
    space.add("x".to_owned());
    {
        let mut scope1 = space.sub_space("s1".to_owned());
        scope1.add("y".to_owned());
        {
            let mut scope2 = scope1.sub_space("s2".to_owned());
            scope2.add("x".to_owned());
            assert_eq!(scope2.get(&path!(x)), Ok(ID::new(2)));
            assert_eq!(scope2.get(&path!(y)), Err(path!(y)));
        }
        scope1.add("y".to_owned());
        assert_eq!(scope1.get(&path!(x)), Err(path!(x)));
        assert_eq!(scope1.get(&path!(y)), Ok(ID::new(3)));
    }
    
    space.add("z".to_owned());
    assert_eq!(space.get(&path!(x)), Ok(ID::new(0)));
    assert_eq!(space.get(&path!(y)), Err(path!(y)));
    assert_eq!(space.get(&path!(z)), Ok(ID::new(4)));

    assert_eq!(space.paths(), [path![x], path![s1::y], path![s1::s2::x], path![s1::y], path![z]]);
}

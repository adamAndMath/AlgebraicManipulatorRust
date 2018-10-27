use std::collections::HashMap;
use std::marker::PhantomData;
use env::{ Namespace, LocalNamespace, ID, LocalID };

#[test]
fn create_empty_space() {
    let space: Namespace<&str> = Namespace::new(HashMap::new());
    LocalNamespace::new(&space);
}

#[test]
fn create_space_with_a_b_c() {
    let space: Namespace<&str> = Namespace::new(vec![
        ("a".to_owned(), ID::Predef(0, PhantomData)),
        ("b".to_owned(), ID::Predef(1, PhantomData)),
        ("c".to_owned(), ID::Predef(2, PhantomData)),
    ]);
    let space = LocalNamespace::new(&space);

    assert_eq!(space.get(&path![a]), Ok(ID::Predef(0, PhantomData).into()));
    assert_eq!(space.get(&path![b]), Ok(ID::Predef(1, PhantomData).into()));
    assert_eq!(space.get(&path![c]), Ok(ID::Predef(2, PhantomData).into()));
}

#[test]
fn add_data_to_empty_space() {
    let space: Namespace<&str> = Namespace::new(HashMap::new());
    {
        let space = LocalNamespace::new(&space);
        let space = space.scope(vec!("a".to_owned(), "b".to_owned(), "c".to_owned()));

        assert_eq!(space.get(&path!(a)), Ok(LocalID::new(0)));
        assert_eq!(space.get(&path!(b)), Ok(LocalID::new(1)));
        assert_eq!(space.get(&path!(c)), Ok(LocalID::new(2)));
    }

    assert_eq!(space.paths().len(), 0);
}

#[test]
fn add_data_in_and_after_scope() {
    let mut space: Namespace<&str> = Namespace::new(vec![]);
    {
        space.add("static".to_owned());
        let space = LocalNamespace::new(&space);
        {
            let scope1 = space.scope(vec!["x".to_owned(), "y".to_owned()]);
            {
                let scope2 = scope1.scope(vec!["x".to_owned()]);
                assert_eq!(scope2.get(&path!(x)), Ok(LocalID::new(0)));
                assert_eq!(scope2.get(&path!(y)), Ok(LocalID::new(2)));
            }
            assert_eq!(scope1.get(&path!(x)), Ok(LocalID::new(0)));
            assert_eq!(scope1.get(&path!(y)), Ok(LocalID::new(1)));
        }

        assert_eq!(space.get(&path!(x)), Err(path!(x)));
        assert_eq!(space.get(&path!(y)), Err(path!(y)));
    }

    assert_eq!(space.paths(), [path![static]]);
}

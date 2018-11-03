use parser::Parse;
use std::marker::PhantomData;
use env::{ Space, Namespace, LocalNamespace, ID, LocalID, Path };

#[test]
fn create_empty_space() {
    let mut names = vec![];
    let space: Namespace<&str> = Namespace::new(&mut names, Space::default());
    LocalNamespace::new(&space);
}

#[test]
fn create_space_with_a_b_c() {
    let mut names = vec![];
    let space: Namespace<&str> = Namespace::new(&mut names, Space::new::<&str,_>(vec![
        ("a", ID::Predef(0, PhantomData)),
        ("b", ID::Predef(1, PhantomData)),
        ("c", ID::Predef(2, PhantomData)),
    ]));
    let space = LocalNamespace::new(&space);

    assert_eq!(space.get(&Path::parse("a")), Ok(ID::Predef(0, PhantomData).into()));
    assert_eq!(space.get(&Path::parse("b")), Ok(ID::Predef(1, PhantomData).into()));
    assert_eq!(space.get(&Path::parse("c")), Ok(ID::Predef(2, PhantomData).into()));
}

#[test]
fn add_data_to_empty_space() {
    let mut names = vec![];
    
    {
        let space: Namespace<&str> = Namespace::new(&mut names, Space::default());
        {
            let space = LocalNamespace::new(&space);
            let space = space.scope(vec!("a", "b", "c"));

            assert_eq!(space.get(&Path::parse("a")), Ok(LocalID::new(0)));
            assert_eq!(space.get(&Path::parse("b")), Ok(LocalID::new(1)));
            assert_eq!(space.get(&Path::parse("c")), Ok(LocalID::new(2)));
        }
    }

    assert_eq!(names.len(), 0);
}

#[test]
fn add_data_in_and_after_scope() {
    let mut names = vec![];
    
    {
        let mut space: Namespace<&str> = Namespace::new(&mut names, Space::default());
        {
            space.add(&"static");
            let space = LocalNamespace::new(&space);
            {
                let scope1 = space.scope(vec!["x", "y"]);
                {
                    let scope2 = scope1.scope(vec!["x"]);
                    assert_eq!(scope2.get(&Path::parse("x")), Ok(LocalID::new(0)));
                    assert_eq!(scope2.get(&Path::parse("y")), Ok(LocalID::new(2)));
                }
                assert_eq!(scope1.get(&Path::parse("x")), Ok(LocalID::new(0)));
                assert_eq!(scope1.get(&Path::parse("y")), Ok(LocalID::new(1)));
            }

            assert_eq!(space.get(&Path::parse("x")), Err(Path::parse("x")));
            assert_eq!(space.get(&Path::parse("y")), Err(Path::parse("y")));
        }
    }

    assert_eq!(names, ["static".to_owned().into()]);
}

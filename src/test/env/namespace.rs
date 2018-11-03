use parser::Parse;
use std::marker::PhantomData;
use env::{ ID, Space, Namespace, Path };

#[test]
#[allow(unused_variables)]
fn create_empty_space() {
    let mut names = vec![];
    let space: Namespace<&str> = Namespace::new(&mut names, Space::default());
}

#[test]
fn create_space_with_a_b_c() {
    let mut names = vec![];
    let space: Namespace<&str> = Namespace::new(&mut names, Space::new::<&str,_>(vec![
        ("a", ID::Predef(0, PhantomData)),
        ("b", ID::Predef(1, PhantomData)),
        ("c", ID::Predef(2, PhantomData)),
    ]));

    println!("{:?}", space);
    assert_eq!(space.get(&Path::parse("a")), Ok(ID::Predef(0, PhantomData)));
    assert_eq!(space.get(&Path::parse("b")), Ok(ID::Predef(1, PhantomData)));
    assert_eq!(space.get(&Path::parse("c")), Ok(ID::Predef(2, PhantomData)));
}

#[test]
fn add_data_to_empty_space() {
    let mut names = vec![];

    {
        let mut space: Namespace<&str> = Namespace::new(&mut names, Space::default());

        space.add(&"a");
        space.add(&"b");
        space.add(&"c");

        assert_eq!(space.get(&Path::parse("a")), Ok(ID::new(0)));
        assert_eq!(space.get(&Path::parse("b")), Ok(ID::new(1)));
        assert_eq!(space.get(&Path::parse("c")), Ok(ID::new(2)));
    }

    assert_eq!(names, [
        "a".to_owned().into(),
        "b".to_owned().into(),
        "c".to_owned().into(),
    ]);
}

#[test]
fn add_data_in_and_after_scope() {
    let mut names = vec![];

    {
        let mut space: Namespace<&str> = Namespace::new(&mut names, Space::new::<&str,_>(vec![("Predef", ID::Predef(0, PhantomData))]));
        space.add(&"x");
        {
            let mut scope1 = space.sub_space(&"s1");
            scope1.add(&"y");
            {
                let mut scope2 = scope1.sub_space(&"s2");
                scope2.add(&"x");
                assert_eq!(scope2.get(&Path::parse("x")), Ok(ID::new(2)));
                assert_eq!(scope2.get(&Path::parse("y")), Err(Path::parse("y")));
            }
            scope1.add(&"y");
            assert_eq!(scope1.get(&Path::parse("x")), Err(Path::parse("x")));
            assert_eq!(scope1.get(&Path::parse("y")), Ok(ID::new(3)));
            assert_eq!(scope1.get(&Path::parse("s2::x")), Ok(ID::new(2)));
        }
        
        space.add(&"z");
        assert_eq!(space.get(&Path::parse("x")), Ok(ID::new(0)));
        assert_eq!(space.get(&Path::parse("y")), Err(Path::parse("y")));
        assert_eq!(space.get(&Path::parse("z")), Ok(ID::new(4)));
        assert_eq!(space.get(&Path::parse("s1::y")), Ok(ID::new(3)));
        assert_eq!(space.get(&Path::parse("s1::s2::x")), Ok(ID::new(2)));
    }

    assert_eq!(names, [
        "x".to_owned().into(),
        Path::new(vec!["s1".to_owned(), "y".to_owned()]),
        Path::new(vec!["s1".to_owned(), "s2".to_owned(), "x".to_owned()]),
        Path::new(vec!["s1".to_owned(), "y".to_owned()]),
        "z".to_owned().into(),
    ]);
}

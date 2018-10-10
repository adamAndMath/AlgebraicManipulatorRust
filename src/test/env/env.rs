use env::{ ID, Env, Path };

#[test]
fn create_empty_env() {
    let mut data = Vec::<&str>::new();
    Env::new(&mut data);
}

#[test]
fn create_env_with_a_b_c() {
    let mut data = vec!["a", "b", "c"];
    let env = Env::new(&mut data);

    assert_eq!(env.get(ID::new(0)), Ok(&"a"));
    assert_eq!(env.get(ID::new(1)), Ok(&"b"));
    assert_eq!(env.get(ID::new(2)), Ok(&"c"));
}

#[test]
fn add_data_to_empty_env() {
    let mut data = Vec::<&str>::new();
    {
        let mut env = Env::new(&mut data);
        env.add("a".to_owned(), "1");
        env.add("b".to_owned(), "2");
        env.add("c".to_owned(), "3");

        assert_eq!(env.get_id(&path!(a)), Ok(ID::new(0)));
        assert_eq!(env.get_id(&path!(b)), Ok(ID::new(1)));
        assert_eq!(env.get_id(&path!(c)), Ok(ID::new(2)));

        assert_eq!(env.get(ID::new(0)), Ok(&"1"));
        assert_eq!(env.get(ID::new(1)), Ok(&"2"));
        assert_eq!(env.get(ID::new(2)), Ok(&"3"));
    }

    assert_eq!(data[..], ["1", "2", "3"]);
}

#[test]
fn add_data_in_and_after_scope() {
    let mut data = vec!("Not named");
    {
        let mut env = Env::new(&mut data);
        env.add("x".to_owned(), "1st");
        {
            let mut scope1 = env.scope();
            scope1.add("y".to_owned(), "2nd");
            {
                let mut scope2 = scope1.scope();
                scope2.add("x".to_owned(), "3rd");
                assert_eq!(scope2.get_id(&path!(x)).map(|id|scope2.get(id)), Ok(Ok(&"3rd")));
                assert_eq!(scope2.get_id(&path!(y)).map(|id|scope2.get(id)), Ok(Ok(&"2nd")));
            }
            scope1.add("y".to_owned(), "4th");
            assert_eq!(scope1.get_id(&path!(x)).map(|id|scope1.get(id)), Ok(Ok(&"1st")));
            assert_eq!(scope1.get_id(&path!(y)).map(|id|scope1.get(id)), Ok(Ok(&"4th")));
        }
        
        env.add("z".to_owned(), "5th");
        assert_eq!(env.get_id(&path!(x)).map(|id|env.get(id)), Ok(Ok(&"1st")));
        assert_eq!(env.get_id(&path!(y)), Err(path!(y)));
        assert_eq!(env.get_id(&path!(z)).map(|id|env.get(id)), Ok(Ok(&"5th")));
    }

    assert_eq!(data[..], ["Not named", "1st", "2nd", "3rd", "4th", "5th"]);
}

#[test]
fn alias_unnamed_data() {
    let mut data = vec!("Not named");
    let mut env = Env::new(&mut data);
    env.alias("name".to_owned(), ID::new(0));
    assert_eq!(env.get_id(&path!(name)), Ok(ID::new(0)));
    assert_eq!(env.get(ID::new(0)), Ok(&"Not named"));
}
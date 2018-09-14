use env::{ ID, Env };

#[test]
fn create_empty_env() {
    let mut data = Vec::<&str>::new();
    Env::new(&mut data);
}

#[test]
fn create_env_with_a_b_c() {
    let mut data = vec!["a", "b", "c"];
    let env = Env::new(&mut data);

    assert_eq!(env.get(ID::new(0)), Some(&"a"));
    assert_eq!(env.get(ID::new(1)), Some(&"b"));
    assert_eq!(env.get(ID::new(2)), Some(&"c"));
}

#[test]
fn add_data_to_empty_env() {
    let mut data = Vec::<&str>::new();
    {
        let mut env = Env::new(&mut data);
        env.add("a".to_owned(), "1");
        env.add("b".to_owned(), "2");
        env.add("c".to_owned(), "3");

        assert_eq!(env.get_id("a"), Some(ID::new(0)));
        assert_eq!(env.get_id("b"), Some(ID::new(1)));
        assert_eq!(env.get_id("c"), Some(ID::new(2)));

        assert_eq!(env.get(ID::new(0)), Some(&"1"));
        assert_eq!(env.get(ID::new(1)), Some(&"2"));
        assert_eq!(env.get(ID::new(2)), Some(&"3"));
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
                assert_eq!(scope2.get_id("x").and_then(|id|scope2.get(id)), Some(&"3rd"));
                assert_eq!(scope2.get_id("y").and_then(|id|scope2.get(id)), Some(&"2nd"));
            }
            scope1.add("y".to_owned(), "4th");
            assert_eq!(scope1.get_id("x").and_then(|id|scope1.get(id)), Some(&"1st"));
            assert_eq!(scope1.get_id("y").and_then(|id|scope1.get(id)), Some(&"4th"));
        }
        
        env.add("z".to_owned(), "5th");
        assert_eq!(env.get_id("x").and_then(|id|env.get(id)), Some(&"1st"));
        assert_eq!(env.get_id("y"), None);
        assert_eq!(env.get_id("z").and_then(|id|env.get(id)), Some(&"5th"));
    }

    assert_eq!(data[..], ["Not named", "1st", "2nd", "3rd", "4th", "5th"]);
}

#[test]
fn alias_unnamed_data() {
    let mut data = vec!("Not named");
    let mut env = Env::new(&mut data);
    env.alias("name".to_owned(), ID::new(0));
    assert_eq!(env.get_id("name"), Some(ID::new(0)));
    assert_eq!(env.get(ID::new(0)), Some(&"Not named"));
}
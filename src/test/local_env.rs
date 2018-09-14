use env::{ id::ID, local_id::LocalID, env::Env, local_env::LocalEnv };

#[test]
fn create_empty_env() {
    let mut data = Vec::<&str>::new();
    let env = Env::new(&mut data);
    LocalEnv::new(&env);
}

#[test]
fn create_env_with_a_b_c() {
    let mut data = vec!["a", "b", "c"];
    let env = Env::new(&mut data);
    let env = LocalEnv::new(&env);

    assert_eq!(env.get(ID::new(0)), Some(&"a"));
    assert_eq!(env.get(ID::new(1)), Some(&"b"));
    assert_eq!(env.get(ID::new(2)), Some(&"c"));
}

#[test]
fn add_data_to_empty_env() {
    let mut data = Vec::<&str>::new();
    {
        let env = Env::new(&mut data);
        let env = LocalEnv::new(&env);
        let env = env.scope(vec!(("a".to_owned(), "1"), ("b".to_owned(), "2"), ("c".to_owned(), "3")));

        assert_eq!(env.get_id("a"), Some(LocalID::new(0)));
        assert_eq!(env.get_id("b"), Some(LocalID::new(1)));
        assert_eq!(env.get_id("c"), Some(LocalID::new(2)));

        assert_eq!(env.get(LocalID::new(0)), Some(&"1"));
        assert_eq!(env.get(LocalID::new(1)), Some(&"2"));
        assert_eq!(env.get(LocalID::new(2)), Some(&"3"));
    }

    assert!(data.is_empty());
}

#[test]
fn add_data_in_and_after_scope() {
    let mut data = vec!("Not named");
    {
        let env = Env::new(&mut data);
        let env = LocalEnv::new(&env);
        {
            let scope1 = env.scope(vec![("x".to_owned(), "1st"), ("y".to_owned(), "2nd")]);
            {
                let scope2 = scope1.scope(vec![("x".to_owned(), "3rd")]);
                assert_eq!(scope2.get_id("x").and_then(|id|scope2.get(id)), Some(&"3rd"));
                assert_eq!(scope2.get_id("y").and_then(|id|scope2.get(id)), Some(&"2nd"));
            }
            assert_eq!(scope1.get_id("x").and_then(|id|scope1.get(id)), Some(&"1st"));
            assert_eq!(scope1.get_id("y").and_then(|id|scope1.get(id)), Some(&"2nd"));
        }

        assert_eq!(env.get_id("x"), None);
        assert_eq!(env.get_id("y"), None);
    }

    assert_eq!(data[..], ["Not named"]);
}

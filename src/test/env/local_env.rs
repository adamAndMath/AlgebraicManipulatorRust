use parser::Parse;
use env::{ ID, LocalID, Env, LocalEnv, Path };

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

    assert_eq!(env.get(ID::new(0)), Ok(&"a"));
    assert_eq!(env.get(ID::new(1)), Ok(&"b"));
    assert_eq!(env.get(ID::new(2)), Ok(&"c"));
}

#[test]
fn add_data_to_empty_env() {
    let mut data = Vec::<&str>::new();
    {
        let env = Env::new(&mut data);
        let env = LocalEnv::new(&env);
        let env = env.scope(vec!(("a", "1"), ("b", "2"), ("c", "3")));

        assert_eq!(env.get_id(&Path::parse("a")), Ok(LocalID::new(0)));
        assert_eq!(env.get_id(&Path::parse("b")), Ok(LocalID::new(1)));
        assert_eq!(env.get_id(&Path::parse("c")), Ok(LocalID::new(2)));

        assert_eq!(env.get(LocalID::new(0)), Ok(&"1"));
        assert_eq!(env.get(LocalID::new(1)), Ok(&"2"));
        assert_eq!(env.get(LocalID::new(2)), Ok(&"3"));
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
            let scope1 = env.scope(vec![("x", "1st"), ("y", "2nd")]);
            {
                let scope2 = scope1.scope(vec![("x", "3rd")]);
                assert_eq!(scope2.get_id(&Path::parse("x")).map(|id|scope2.get(id)), Ok(Ok(&"3rd")));
                assert_eq!(scope2.get_id(&Path::parse("y")).map(|id|scope2.get(id)), Ok(Ok(&"2nd")));
            }
            assert_eq!(scope1.get_id(&Path::parse("x")).map(|id|scope1.get(id)), Ok(Ok(&"1st")));
            assert_eq!(scope1.get_id(&Path::parse("y")).map(|id|scope1.get(id)), Ok(Ok(&"2nd")));
        }

        assert_eq!(env.get_id(&Path::parse("x")), Err(Path::parse("x")));
        assert_eq!(env.get_id(&Path::parse("y")), Err(Path::parse("y")));
    }

    assert_eq!(data[..], ["Not named"]);
}

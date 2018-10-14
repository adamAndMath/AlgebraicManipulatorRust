use std::marker::PhantomData;
use env::{ ID, LocalID, Env, LocalEnv, Path, EnvData };

#[test]
fn create_empty_env() {
    let mut data = EnvData::new(Vec::<&str>::new());
    let env = Env::new(&mut data);
    LocalEnv::new(&env);
}

#[test]
fn create_env_with_a_b_c() {
    let mut data = EnvData::new(vec!["a", "b", "c"]);
    let env = Env::new(&mut data);
    let env = LocalEnv::new(&env);

    assert_eq!(env.get(ID::Predef(0, PhantomData)), &"a");
    assert_eq!(env.get(ID::Predef(1, PhantomData)), &"b");
    assert_eq!(env.get(ID::Predef(2, PhantomData)), &"c");
}

#[test]
fn add_data_to_empty_env() {
    let mut data = EnvData::new(Vec::<&str>::new());
    {
        let env = Env::new(&mut data);
        let env = LocalEnv::new(&env);
        let env = env.scope(vec!(("a".to_owned(), "1"), ("b".to_owned(), "2"), ("c".to_owned(), "3")));

        assert_eq!(env.get_id(&path!(a)), Ok(LocalID::new(0)));
        assert_eq!(env.get_id(&path!(b)), Ok(LocalID::new(1)));
        assert_eq!(env.get_id(&path!(c)), Ok(LocalID::new(2)));

        assert_eq!(env.get(LocalID::new(0)), &"1");
        assert_eq!(env.get(LocalID::new(1)), &"2");
        assert_eq!(env.get(LocalID::new(2)), &"3");
    }

    assert_eq!(data.len(), 0);
}

#[test]
fn add_data_in_and_after_scope() {
    let mut data = EnvData::new(vec![]);
    {
        let mut env = Env::new(&mut data);
        env.add("static".to_owned(), "Static");
        let env = LocalEnv::new(&env);
        {
            let scope1 = env.scope(vec![("x".to_owned(), "1st"), ("y".to_owned(), "2nd")]);
            {
                let scope2 = scope1.scope(vec![("x".to_owned(), "3rd")]);
                assert_eq!(scope2.get_id(&path!(x)).map(|id|scope2.get(id)), Ok(&"3rd"));
                assert_eq!(scope2.get_id(&path!(y)).map(|id|scope2.get(id)), Ok(&"2nd"));
            }
            assert_eq!(scope1.get_id(&path!(x)).map(|id|scope1.get(id)), Ok(&"1st"));
            assert_eq!(scope1.get_id(&path!(y)).map(|id|scope1.get(id)), Ok(&"2nd"));
        }

        assert_eq!(env.get_id(&path!(x)), Err(path!(x)));
        assert_eq!(env.get_id(&path!(y)), Err(path!(y)));
    }

    assert_eq!(data[..], ["Static"]);
}

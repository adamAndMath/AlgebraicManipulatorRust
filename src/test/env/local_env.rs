use std::marker::PhantomData;
use env::{ ID, LocalID, Env, LocalEnv };

#[test]
fn create_empty_env() {
    let env = Env::new(Vec::<&str>::new());
    LocalEnv::new(&env);
}

#[test]
fn create_env_with_a_b_c() {
    let env = Env::new(vec!["a", "b", "c"]);
    let env = LocalEnv::new(&env);

    assert_eq!(env[ID::Predef(0, PhantomData)], "a");
    assert_eq!(env[ID::Predef(1, PhantomData)], "b");
    assert_eq!(env[ID::Predef(2, PhantomData)], "c");
}

#[test]
fn add_data_to_empty_env() {
    let env = Env::new(Vec::<&str>::new());
    {
        let env = LocalEnv::new(&env);
        let env = env.scope(vec!("1", "2", "3"));

        assert_eq!(env[LocalID::new(0)], "1");
        assert_eq!(env[LocalID::new(1)], "2");
        assert_eq!(env[LocalID::new(2)], "3");
    }

    assert_eq!(env.len(), 0);
}

#[test]
fn add_data_in_and_after_scope() {
    let mut env = Env::new(vec![]);
    {
        env.add("Static");
        let env = LocalEnv::new(&env);
        {
            let scope1 = env.scope(vec!["1st", "2nd"]);
            {
                let scope2 = scope1.scope(vec!["3rd"]);
                assert_eq!(scope2[LocalID::new(0)], "3rd");
                assert_eq!(scope2[LocalID::new(1)], "1st");
                assert_eq!(scope2[LocalID::new(2)], "2nd");
            }
            assert_eq!(scope1[LocalID::new(0)], "1st");
            assert_eq!(scope1[LocalID::new(1)], "2nd");
        }
    }

    assert_eq!(env[..], ["Static"]);
}

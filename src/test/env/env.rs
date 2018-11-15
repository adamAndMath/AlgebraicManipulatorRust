use std::marker::PhantomData;
use env::{ ID, Env };

#[test]
fn create_empty_env() {
    Env::new(&mut Vec::<&str>::new());
}

#[test]
fn create_env_with_a_b_c() {
    let mut data = vec!["a", "b", "c"];
    let env = Env::new(&mut data);

    assert_eq!(env[ID::Predef(0, PhantomData)], "a");
    assert_eq!(env[ID::Predef(1, PhantomData)], "b");
    assert_eq!(env[ID::Predef(2, PhantomData)], "c");

    {
        let env = env.scope(vec![]);
        assert_eq!(env[ID::Predef(0, PhantomData)], "a");
        assert_eq!(env[ID::Predef(1, PhantomData)], "b");
        assert_eq!(env[ID::Predef(2, PhantomData)], "c");
    }
}

#[test]
fn add_data_to_empty_env() {
    let mut data = Vec::<&str>::new();
    let mut env = Env::new(&mut data);
    env.add("1");
    env.add("2");
    env.add("3");

    assert_eq!(env[ID::new(0)], "1");
    assert_eq!(env[ID::new(1)], "2");
    assert_eq!(env[ID::new(2)], "3");

    assert_eq!(env[..], ["1", "2", "3"]);

    {
        let env = env.scope(vec![]);
        assert_eq!(env[ID::Normal(0, 1, PhantomData)], "1");
        assert_eq!(env[ID::Normal(1, 1, PhantomData)], "2");
        assert_eq!(env[ID::Normal(2, 1, PhantomData)], "3");

        assert_eq!(env[..], Vec::<&str>::new()[..]);
    }
}

#[test]
fn add_data_to_empty_scope() {
    let mut data = Vec::<&str>::new();
    let env = Env::new(&mut data);

    {
        let mut env = env.scope(vec![]);
        env.add("1");
        env.add("2");
        env.add("3");

        assert_eq!(env[ID::new(0)], "1");
        assert_eq!(env[ID::new(1)], "2");
        assert_eq!(env[ID::new(2)], "3");

        assert_eq!(env[..], ["1", "2", "3"]);
    }

    assert_eq!(env[..], Vec::<&str>::new()[..]);
}

#[test]
fn add_data_in_and_after_scope() {
    let mut data = Vec::<&str>::new();
    let mut env = Env::new(&mut data);
    env.add("Static");
    {
        let mut scope1 = env.scope(vec!["1st", "2nd"]);
        {
            let scope2 = scope1.scope(vec!["3rd"]);
            assert_eq!(scope2[ID::new(0)], "3rd");
            assert_eq!(scope2[ID::Normal(0, 1, PhantomData)], "1st");
            assert_eq!(scope2[ID::Normal(1, 1, PhantomData)], "2nd");
            assert_eq!(scope2[ID::Normal(0, 2, PhantomData)], "Static");
            
            assert_eq!(scope2[..], ["3rd"]);
        }
        scope1.add("4th");
        assert_eq!(scope1[ID::new(0)], "1st");
        assert_eq!(scope1[ID::new(1)], "2nd");
        assert_eq!(scope1[ID::new(2)], "4th");
        assert_eq!(scope1[ID::Normal(0, 1, PhantomData)], "Static");
        
        assert_eq!(scope1[..], ["1st", "2nd", "4th"]);
    }

    assert_eq!(env[..], ["Static"]);
}

use std::marker::PhantomData;
use env::{ ID, Env };

#[test]
fn create_empty_env() {
    Env::new(Vec::<&str>::new());
}

#[test]
fn create_env_with_a_b_c() {
    let env = Env::new(vec!["a", "b", "c"]);

    assert_eq!(env[ID::Predef(0, PhantomData)], "a");
    assert_eq!(env[ID::Predef(1, PhantomData)], "b");
    assert_eq!(env[ID::Predef(2, PhantomData)], "c");
}

#[test]
fn add_data_to_empty_env() {
    let mut env = Env::new(vec![]);
    env.add("1");
    env.add("2");
    env.add("3");

    assert_eq!(env[ID::new(0)], "1");
    assert_eq!(env[ID::new(1)], "2");
    assert_eq!(env[ID::new(2)], "3");

    assert_eq!(env[..], ["1", "2", "3"]);
}

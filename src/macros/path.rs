macro_rules! path {
    ($($n:ident)::+) => (Path::new(vec![$(stringify!($n).to_owned()),+]));
}
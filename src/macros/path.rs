macro_rules! path {
    ($($n:ident)::+) => (::env::Path::new(vec![$(stringify!($n).to_owned()),+]));
}
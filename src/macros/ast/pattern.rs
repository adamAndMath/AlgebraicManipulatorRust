macro_rules! pattern {
    ($v:ident: $($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) => (Pattern::Var(stringify!($v).to_owned(), ttype!($($t)*$([$($g)*])*$(($($p)*))*)));
    ($($a:ident)::+) => (Pattern::Atom(path!($($a)::+), vec![]));
    ($($a:ident)::+[$($g:tt)*]) => (Pattern::Atom(path!($($a)::+), ttype_vec!($($g)*)));
    ($($f:ident)::+($($p:tt)*)) => (Pattern::Comp(path!($($f)::+), vec![], Box::new(pattern_tuple!($($p)*))));
    ($($f:ident)::+[$($g:tt)*]($($p:tt)*)) => (Pattern::Comp(path!($($f)::+), ttype_vec!($($g)*), Box::new(pattern_tuple!($($p)*))));
    (($($t:tt)*)) => (pattern_tuple!($($t)*))
}

macro_rules! pattern_tuple {
    ($($x:ident):*$(($($p:tt)*))*$([$($g:tt)*])*) => (pattern!($($x):*$(($($p)*))*$([$($g)*])*));
    ($($($x:ident):*$(($($p:tt)*))*$([$($g:tt)*])*),*) => (Pattern::Tuple(vec![$(pattern!($($x):*$(($($p)*))*$([$($g)*])*)),*]));
}

macro_rules! pattern {
    ($v:ident: $($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) => (Pattern::Var(stringify!($v).to_owned(), ttype!($($t)*$([$($g)*])*$(($($p)*))*)));
    ($a:ident) => (Pattern::Atom(stringify!($a).to_owned(), vec![]));
    ($a:ident[$($g:tt)*]) => (Pattern::Atom(stringify!($a).to_owned(), ttype_vec!($($g)*)));
    ($f:ident($($p:tt)*)) => (Pattern::Comp(stringify!($f).to_owned(), vec![], Box::new(pattern_tuple!($($p)*))));
    ($f:ident[$($g:tt)*]($($p:tt)*)) => (Pattern::Comp(stringify!($f).to_owned(), ttype_vec!($($g)*), Box::new(pattern_tuple!($($p)*))));
    (($($t:tt)*)) => (pattern_tuple!($($t)*))
}

macro_rules! pattern_tuple {
    ($($x:ident):*$(($($p:tt)*))*$([$($g:tt)*])*) => (pattern!($($x):*$(($($p)*))*$([$($g)*])*));
    ($($($x:ident):*$(($($p:tt)*))*$([$($g:tt)*])*),*) => (Pattern::Tuple(vec![$(pattern!($($x):*$(($($p)*))*$([$($g)*])*)),*]));
}

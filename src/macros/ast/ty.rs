macro_rules! ttype {
    ($t:ident) => (Type::Gen(stringify!($t).to_owned(), vec!()));
    ($t:ident[$($g:tt)*]) => (Type::Gen(stringify!($t).to_owned(), ttype_vec!($($g)*)));
    (($($p:tt)*)) => (ttype_tuple!($($p)*));
}

macro_rules! ttype_vec {
    ($($($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*),*) =>
        (vec![$(ttype!($($t)*$([$($g)*])*$(($($p)*))*)),*]);
}

macro_rules! ttype_tuple {
    ($($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) =>
        (ttype!($($t)*$([$($g)*])*$(($($p)*))*));
    ($($($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*),*) =>
        (Type::Tuple(vec![$(ttype!($($t)*$([$($g)*])*$(($($p)*))*)),*]));
}

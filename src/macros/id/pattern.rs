macro_rules! pattern_id {
    ($n:ident:$($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) => (PatternID::Var(stringify!($n).to_owned(), type_id!($($t)*$([$($g)*])*$(($($p)*))*)));
    ($a:ident) => (PatternID::Atom($a, vec![]));
    ($a:ident[$($g:tt)*]) => (PatternID::Atom($a, type_id_vec!($($g)*)));
    ($f:ident($($p:tt)*)) => (PatternID::Comp($f, vec![], Box::new(pattern_id_tuple!(() $($p)*,))));
    ($f:ident[$($g:tt)*]($($p:tt)*)) => (PatternID::Comp($f, type_id_vec!($($g)*), Box::new(pattern_id_tuple!(() $($p)*,))));
    (($($t:tt)*)) => (pattern_id_tuple!(() $($p)*,))
}

macro_rules! pattern_id_tuple {
    ((pattern_id!($($e:tt)*), )$(,)* ) => (pattern_id!($($e)*));
    (($($v:tt)*), ) => (PatternID::Tuple(vec!($($v)*)));
    (($($v:tt)*) $n:ident:$($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) => (pattern_id_tuple!(($($v)* pattern_id!($n:$($t)*$([$($g)*])*$(($($p)*))*), ) $($rest)*));
    (($($v:tt)*) $($x:ident)*$(($($p:tt)*))*, $($rest:tt)*) => (pattern_id_tuple!(($($v)* pattern_id!($($x)*$(($($p)*))*), ) $($rest)*));
}

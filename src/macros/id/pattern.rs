macro_rules! pattern_id {
    (+$($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) => (PatternID::Var(type_id!($($t)*$([$($g)*])*$(($($p)*))*)));
    ($a:ident) => (PatternID::Atom($a.clone(), vec![]));
    ($a:ident[$($g:tt)*]) => (PatternID::Atom($a.clone(), type_id_vec!($($g)*)));
    ($f:ident($($p:tt)*)) => (PatternID::Comp($f.clone(), vec![], Box::new(pattern_id_tuple!(() $($p)*,))));
    ($f:ident[$($g:tt)*]($($p:tt)*)) => (PatternID::Comp($f.clone(), type_id_vec!($($g)*), Box::new(pattern_id_tuple!(() $($p)*,))));
    (($($t:tt)*)) => (pattern_id_tuple!(() $($t)*,));
}

macro_rules! pattern_id_tuple {
    ((pattern_id!($($e:tt)*), )$(,)* ) => (pattern_id!($($e)*));
    (($($v:tt)*), ) => (PatternID::Tuple(vec!($($v)*)));
    (($($v:tt)*) +$($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) => (pattern_id_tuple!(($($v)* pattern_id!(+$($t)*$([$($g)*])*$(($($p)*))*), ) $($rest)*));
    (($($v:tt)*) $($x:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) => (pattern_id_tuple!(($($v)* pattern_id!($($x)*$([$g])*$(($($p)*))*), ) $($rest)*));
}

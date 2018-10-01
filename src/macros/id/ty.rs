macro_rules! type_id {
    ($t:ident) => (TypeID::Gen($t.into(), vec!()));
    ($t:ident[$($g:tt)*]) => (TypeID::Gen($t.into(), type_id_gen!(() $($g)*,)));
    (($($p:tt)*)) => (type_id_tuple!($($p)*));
}

macro_rules! type_id_vec {
    ($($($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*),*) => (vec![$(type_id!($($t)*$([$($g)*])*$(($($p)*))*))*]);
}

macro_rules! type_id_gen {
    (($($v:tt)*)$(,)* ) => (vec!($($v)*));
    (($($v:tt)*)  $($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) =>
        (type_id_gen!(($($v)* (Variance::Invariant,     type_id!($($t)*$([$($g)*])*$(($($p)*))*)), ) $($rest)*));
    (($($v:tt)*) +$($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) =>
        (type_id_gen!(($($v)* (Variance::Covariant,     type_id!($($t)*$([$($g)*])*$(($($p)*))*)), ) $($rest)*));
    (($($v:tt)*) -$($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) =>
        (type_id_gen!(($($v)* (Variance::Contravariant, type_id!($($t)*$([$($g)*])*$(($($p)*))*)), ) $($rest)*));
}

macro_rules! type_id_tuple {
    ($($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) => (type_id!($($t)*$([$($g)*])*$(($($p)*))*));
    ($($($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*),*) => (TypeID::Tuple(vec![$(type_id!($($t)*$([$($g)*])*$(($($p)*))*)),*]));
}

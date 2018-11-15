macro_rules! type_id {
    ($t:ident) => (TypeID::Gen($t.clone(), vec!()));
    ($t:ident[$($g:tt)*]) => (TypeID::Gen($t.clone(), type_id_vec!($($g)*)));
    (($($p:tt)*)) => (type_id_tuple!($($p)*));
}

macro_rules! type_id_vec {
    ($($($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*),*) => (vec![$(type_id!($($t)*$([$($g)*])*$(($($p)*))*)),*]);
}

macro_rules! type_id_tuple {
    ($($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) => (type_id!($($t)*$([$($g)*])*$(($($p)*))*));
    ($($($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*),*) => (TypeID::Tuple(vec![$(type_id!($($t)*$([$($g)*])*$(($($p)*))*)),*]));
}

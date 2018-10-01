macro_rules! exp_id {
    ($x:ident) => (ExpID::Var($x.into(), vec![]));
    ($x:ident[$($g:tt)*]) => (ExpID::Var($x.into(), type_id_vec!($($g)*)));
    ($x:ident$([$($g:tt)*])*($($p:tt)*)) => (ExpID::Call(Box::new(exp_id!($x$([$($g)*])*)), Box::new(exp_id_tuple!($($p)*))));
    (($($p:tt)*)) => (exp_id_tuple!($($p)*));
}

macro_rules! exp_id_tuple {
    ($($x:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) => (exp_id!($($x)*$([$($g)*])*$(($($p)*))*));
    ($($($x:ident)*$([$($g:tt)*])*$(($($p:tt)*))*),*) => (ExpID::Tuple(vec![$(exp_id!($($x)*$([$($g)*])*$(($($p)*))*)),*]));
}

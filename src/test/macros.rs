macro_rules! ttype {
    ($t:ident) => (Type::Gen(stringify!($t).to_owned(), vec!()));
    ($t:ident[$($g:tt)*]) => (Type::Gen(stringify!($t).to_owned(), ttype_vec!((), $($g)*)));
    ((($p:tt)*)>) => (Type::Tuple(ttype_vec!((), $($p)*)));
}

macro_rules! ttype_vec {
    (($($v:tt)*), ) => (vec!($($v)*));
    (($($v:tt)*), $t:ident) => (ttype_vec!(($($v)* ttype!($t)), ));
    (($($v:tt)*), $t:ident[$($g:tt)*]) => (ttype_vec!(($($v)* ttype!($t[$($g)*])), ));
    (($($v:tt)*), ($($t:tt)*)) => (ttype_vec!(($($v)* ttype!(($($t),*))), ));
    (($($v:tt)*), $t:ident, $($rest:tt)*) => (ttype_vec!(($($v)* ttype!($t), ), $($rest)*));
    (($($v:tt)*), $t:ident[$($g:tt)*], $($rest:tt)*) => (ttype_vec!(($($v)* ttype!($t[$($g)*])), $($rest)*));
    (($($v:tt)*), ($($p:tt)*), $($rest:tt)*) => (ttype_vec!(($($v)* ttype!(($($p),*))), $($rest)*));
}

macro_rules! exp {
    ($x:ident) => (Exp::Var(stringify!($x).to_owned()));
    ($x:ident($($p:tt)*)) => (Exp::Call(Box::new(exp!($x)), Box::new(exp_tuple!((), $($p)*))));
    (($($p:tt)*)) => (exp_tuple!((), $p));
}

macro_rules! exp_tuple {
    ((exp!($($e:tt)*)), ) => (exp!($($e)*));
    (($($v:tt)*), ) => (Exp::Tuple(vec!($($v)*)));
    (($($v:tt)*), $x:ident$(($($p:tt)*))*) => (exp_tuple!(($($v)* exp!($x$(($($p)*))*)), ));
    (($($v:tt)*), $(($($p:tt)*))+) => (exp_tuple!(($($v)* exp!($(($($p)*))*)), ));
    (($($v:tt)*), $x:ident$(($($p:tt)*))*, $($rest:tt)*) => (exp_tuple!(($($v)* exp!($x$(($($p)*))*)), $($rest)*));
    (($($v:tt)*), $(($($p:tt)*))+) => (exp_tuple!(($($v)* exp!($(($($p)*))*)), $($rest)*));
}
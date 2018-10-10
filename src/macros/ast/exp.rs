macro_rules! exp {
    ($($x:ident)::+) => (Exp::Var(path!($($x)::+), vec![]));
    ($($x:ident)::+[$($g:tt)*]) => (Exp::Var(path!($($x)::+).to_owned(), ttype_vec!($($g)*)));
    ({$($m:tt)*}$(($($p:tt)*))*) => (exp_call!(Exp::Closure(exp_match!($($m)*)), $(($($p)*))*));
    ($($x:ident):*$(($($p:tt)*))* -> $($rest:tt)*) => (Exp::Closure(vec![(pattern!($($x):*$(($($p)*))*), exp!($($rest)*))]));
    ($($x:ident)::+$([$($g:tt)*])*$(($($p:tt)*))+) => (exp_call!(exp!($($x)::+$([$($g)*])*), $(($($p)*))+));
    (($($t:tt)*)$(($($p:tt)*))*) => (exp_call!(exp_tuple!($($t)*), $(($($p)*))*));
}

macro_rules! exp_call {
    ($current:expr, ) => ($current);
    ($current:expr, ($($p:tt)*)$($rest:tt)*) => (exp_call!(Exp::Call(Box::new($current), Box::new(exp_tuple!($($p)*))), $($rest)*));
}

macro_rules! exp_match {
    ($($($px:ident)::*$(($($pp:tt)*))* => $($ex:ident)::*$([$($eg:tt)*])*$(($($ep:tt)*))*),*) =>
        (vec![$((pattern!($($px)::*$(($($pp)*))*), exp!($($ex)::*$([$($eg)*])*$(($($ep)*))*))),*]);
}

macro_rules! exp_tuple {
    ($($({$($m:tt)*})*$($x:ident):*$([$($g:tt)*])*$(($($p:tt)*))*)->*) => (exp!($($({$($m)*})*$($x):*$([$($g)*])*$(($($p)*))*)->*));
    ($($($({$($m:tt)*})*$($x:ident):*$([$($g:tt)*])*$(($($p:tt)*))*)->*),*) => (Exp::Tuple(vec![$(exp!($($({$($m)*})*$($x):*$([$($g)*])*$(($($p)*))*)->*)),*]));
}

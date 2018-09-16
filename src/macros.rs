macro_rules! ttype {
    ($t:ident) => (Type::Gen(stringify!($t).to_owned(), vec!()));
    ($t:ident[$($g:tt)*]) => (Type::Gen(stringify!($t).to_owned(), ttype_vec!(() $($g)*,)));
    (($($p:tt)*)) => (Type::Tuple(ttype_vec!(() $($p)*,)));
}

macro_rules! ttype_vec {
    (($($v:tt)*)$(,)* ) => (vec!($($v)*));
    (($($v:tt)*) $($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) =>
        (ttype_vec!(($($v)* ttype!($($t)*$([$($g)*])*$(($($p)*))*), ) $($rest)*));
}

macro_rules! type_id {
    ($t:ident) => (TypeID::Gen($t.into(), vec!()));
    ($t:ident[$($g:tt)*]) => (TypeID::Gen($t.into(), type_id_gen!(() $($g)*,)));
    (($($p:tt)*)) => (type_id_tuple!(() $($p)*,));
}

macro_rules! type_id_vec {
    (($($v:tt)*)$(,)* ) => (vec!($($v)*));
    (($($v:tt)*) $($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) =>
        (type_id_gen!(($($v)* type_id!($($t)*$([$($g)*])*$(($($p)*))*), ) $($rest)*));
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
    ((type_id!($($e:tt)*), )$(,)* ) => (type_id!($($e)*));
    (($($v:tt)*)$(,)* ) => (TypeID::Tuple(vec!($($v)*)));
    (($($v:tt)*) $($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) =>
        (type_id_tuple!(($($v)* type_id!($($t)*$([$($g)*])*$(($($p)*))*), ) $($rest)*));
}

macro_rules! exp {
    ($x:ident) => (Exp::Var(stringify!($x).to_owned(), vec![]));
    ($x:ident[$($g:tt)*]) => (Exp::Var(stringify!($x).to_owned(), ttype_vec!(() $($g)*,)));
    (match($($x:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, {$($m:tt)*})) =>
        (Exp::Match(Box::new(exp!($($x)*$([$($g)*])*$(($($p)*))*)), exp_match!($($m)*)));
    ($($x:ident):*$(($($p:tt)*))* -> $($rest:tt)*) => (Exp::Lambda(pattern!($($x):*$(($($p)*))*), Box::new(exp!($($rest)*))));
    ($x:ident$([$($g:tt)*])*($($p:tt)*)) => (Exp::Call(Box::new(exp!($x$([$($g)*])*)), Box::new(exp_tuple!(() $($p)*,))));
    (($($p:tt)*)) => (exp_tuple!(() $($p)*,));
}

macro_rules! exp_match {
    ($($($px:ident)*$(($($pp:tt)*))* => $($ex:ident)*$([$($eg:tt)*])*$(($($ep:tt)*))*),*) =>
        (vec![$((pattern!($($px)*$(($($pp)*))*), exp!($($ex)*$([$($eg)*])*$(($($ep)*))*))),*]);
}

macro_rules! exp_tuple {
    ((exp!($($e:tt)*), )$(,)* ) => (exp!($($e)*));
    (($($v:tt)*)$(,)* ) => (Exp::Tuple(vec!($($v)*)));
    (($($v:tt)*) $($($x:ident):*$([$($g:tt)*])*$(($($p:tt)*))*)->*, $($rest:tt)*) =>
        (exp_tuple!(($($v)* exp!($($($x)*$([$($g)*])*$(($($p)*))*)->*), ) $($rest)*));
}

macro_rules! exp_id {
    ($x:ident) => (ExpID::Var($x.into(), vec![]));
    ($x:ident[$($g:tt)*]) => (ExpID::Var($x.into(), type_id_vec!(() $($g)*)));
    ($x:ident$([$($g:tt)*])*($($p:tt)*)) => (ExpID::Call(Box::new(exp_id!($x$([$($g)*])*)), Box::new(exp_id_tuple!(() $($p)*,))));
    (($($p:tt)*)) => (exp_id_tuple!(() $($p)*,));
}

macro_rules! exp_id_tuple {
    ((exp_id!($($e:tt)*), )$(,)* ) => (exp_id!($($e)*));
    (($($v:tt)*), ) => (ExpID::Tuple(vec!($($v)*)));
    (($($v:tt)*) $($x:ident)*$([$($g:tt)*])*$(($($p:tt)*))*, $($rest:tt)*) => (exp_id_tuple!(($($v)* exp_id!($($x)*$([$($g)*])*$(($($p)*))*), ) $($rest)*));
}

macro_rules! pattern {
    ($v:ident: $($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) => (Pattern::Var(stringify!($v).to_owned(), ttype!($($t)*$([$($g)*])*$(($($p)*))*)));
    ($a:ident) => (Pattern::Atom(stringify!($a).to_owned()));
    ($f:ident($($p:tt)*)) => (Pattern::Comp(stringify!($f).to_owned(), Box::new(pattern_tuple!(() $($p)*,))));
    (($($t:tt)*)) => (pattern_tuple!(() $($t)*,))
}

macro_rules! pattern_tuple {
    ((pattern!($($e:tt)*), )$(,)* ) => (pattern!($($e)*));
    (($($v:tt)*)$(,)* ) => (Pattern::Tuple(vec!($($v)*)));
    (($($v:tt)*) $($x:ident):*$(($($p:tt)*))*, $($rest:tt)*) => (pattern_tuple!(($($v)* pattern!($($x):*$(($($p)*))*), ) $($rest)*));
}

macro_rules! pattern_id {
    (+$($t:ident)*$([$($g:tt)*])*$(($($p:tt)*))*) => (PatternID::Var(type_id!($($t)*$([$($g)*])*$(($($p)*))*)));
    ($a:ident) => (PatternID::Atom($a));
    ($f:ident($($p:tt)*)) => (PatternID::Comp($f, Box::new(pattern_id_tuple!(() $($p)*,))));
    (($($t:tt)*)) => (pattern_id_tuple!(() $($p)*,))
}

macro_rules! pattern_id_tuple {
    ((pattern_id!($($e:tt)*), )$(,)* ) => (pattern_id!($($e)*));
    (($($v:tt)*), ) => (PatternID::Tuple(vec!($($v)*)));
    (($($v:tt)*) +$x:ident, $($rest:tt)*) => (pattern_id_tuple!(($($v)* pattern_id!(+$x), ) $($rest)*));
    (($($v:tt)*) $($x:ident)*$(($($p:tt)*))*, $($rest:tt)*) => (pattern_id_tuple!(($($v)* pattern_id!($($x)*$(($($p)*))*), ) $($rest)*));
}

macro_rules! tree {
    ($f:tt$(,$t:tt)+) => (tree!($f)$(+tree!($t))*);
    ([$($f:tt),+$(|$($t:tt),+)*]) => (tree!($($f),+)$(*tree!($($t),+))*);
    ($f:tt) => (Tree::edge($f));
}

macro_rules! element {
    (struct $n:ident) => (Element::Struct(stringify!($n).to_owned(), vec![], vec![]));
    (struct $n:ident[$(g:tt)*]) =>
        (Element::Struct(stringify!($n).to_owned(), element_gen!(() $($g)*,), vec![]));
    (struct $n:ident($($v:tt)*)) =>
        (Element::Struct(stringify!($n).to_owned(), vec![], ttype_vec!(() $($v)*,)));
    (struct $n:ident[$($g:tt)*]($($v:tt)*)) =>
        (Element::Struct(stringify!($n).to_owned(), element_gen!(() $($g)*,), ttype_vec!(() $($v)*,)));
    (enum $n:ident { $($v:tt)* }) =>
        (Element::Enum(stringify!($n).to_owned(), vec![], enum_variants!(() $($v)*,)));
    (enum $n:ident[$($g:tt)*] { $($v:tt)* }) =>
        (Element::Enum(stringify!($n).to_owned(), element_gen!(() $($g)*,), enum_variants!(() $($v)*,)));
    (let $n:ident = $($e:tt)*) =>
        (Element::Let(stringify!($n).to_owned(), vec![], None, exp!($($e)*)));
    (let $n:ident[$($g:tt)*] = $($e:tt)*) =>
        (Element::Let(stringify!($n).to_owned(), element_gen!(() $($g)*,), None, exp!($($e)*)));
    (let $n:ident: $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))* = $($e:tt)*) =>
        (Element::Let(stringify!($n).to_owned(), vec![], Some(ttype!($($t)*$([$($gs)*])*$(($($ps)*))*)), exp!($($e)*)));
    (let $n:ident[$($g:tt)*]: $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))* = $($e:tt)*) =>
        (Element::Let(stringify!($n).to_owned(), element_gen!(() $($g)*,), Some(ttype!($($t)*$([$($gs)*])*$(($($ps)*))*)), exp!($($e)*)));
    (fn $n:ident($($p:tt)*) = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![], element_par!($($p)*), None, exp!($($rest)*)));
    (fn $n:ident[$($g:ident),*]($($p:tt)*) = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], element_par!($($p)*), None, exp!($($rest)*)));
    (fn $n:ident($($p:tt)*) -> $($t:ident)*$([$($tg:tt)*])*$(($($tp:tt)*))* = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![], element_par!($($p)*), Some(ttype!($($t)*$([$($tg)*])*$(($($tp)*))*)), exp!($($rest)*)));
    (fn $n:ident[$($g:tt)*]($($p:tt)*) -> $($t:ident)*$([$($tg:tt)*])*$(($($tp:tt)*))* = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], element_par!($($p)*), Some(ttype!($($t)*$([$($tg)*])*$(($($tp)*))*)), exp!($($rest)*)));
}

macro_rules! element_gen {
    (($($v:tt)*), ) => (vec!($($v)*));
    (($($v:tt)*)  $x:ident, $($rest:tt)*) => (element_gen!(($($v)* (Variance::Invariant    , stringify!($x).to_owned())), ) $($rest)*);
    (($($v:tt)*) +$x:ident, $($rest:tt)*) => (element_gen!(($($v)* (Variance::Covariant    , stringify!($x).to_owned())), ) $($rest)*);
    (($($v:tt)*) -$x:ident, $($rest:tt)*) => (element_gen!(($($v)* (Variance::Contravariant, stringify!($x).to_owned())), ) $($rest)*);
}

macro_rules! element_par {
    ($($p:ident : $($t:ident)*$([$($tg:tt)*])*$(($($tp:tt)*))*),*) => (vec![$((stringify!($p).to_owned(), ttype!($($t)*$([$($tg)*])*$(($($tp)*))*))),*]);
}

macro_rules! enum_variants {
    (($($v:tt)*)$(,)* ) => (vec!($($v)*));
    (($($v:tt)*) $n:ident, $($rest:tt)*) => (enum_variants!(($($v)* (stringify!($n).to_owned(), vec!()),) $($rest)*));
    (($($v:tt)*) $n:ident($($t:tt)*), $($rest:tt)*) => (enum_variants!(($($v)* (stringify!($n).to_owned(), ttype_vec!(() $($t)*,)),) $($rest)*));
}

macro_rules! script {
    ($env:ident, ) => ();
    ($env:ident, struct $n:ident$([$($g:tt)*])*$(($($p:tt)*))*; $($rest:tt)*) => {
        element!(struct $n$([$($g)*])*$(($($p)*))*).define(&mut $env).unwrap();
        script!($env, $($rest)*);
    };
    ($env:ident, enum $n:ident$([$($g:tt)*])*{$($v:tt)*} $($rest:tt)*) => {
        element!(enum $n$([$($g)*])*{$($v)*}).define(&mut $env).unwrap();
        script!($env, $($rest)*);
    };
    ($env:ident, let $n:ident$([$($g:tt)*])*$(: $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))*)* = $($($ex:ident):*$([$($eg:tt)*])*$(($($ep:tt)*))*)->*; $($rest:tt)*) => {
        element!(let $n$([$($g)*])*$(: $($t)*$([$($gs)*])*$(($($ps)*))*)* = $($($ex)*$([$($eg)*])*$(($($ep)*))*)->*).define(&mut $env).unwrap();
        script!($env, $($rest)*);
    };
    ($env:ident, fn $n:ident$([$($g:tt)*])*($($p:tt)*)$(-> $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))*)* = $($($ex:ident):*$([$($eg:tt)*])*$(($($ep:tt)*))*)->*; $($rest:tt)*) => {
        element!(fn $n$([$($g)*])*($($p)*)$(-> $($t)*$([$($gs)*])*$(($($ps)*))*)* = $($($ex)*$([$($eg)*])*$(($($ep)*))*)->*).define(&mut $env).unwrap();
        script!($env, $($rest)*);
    };
}

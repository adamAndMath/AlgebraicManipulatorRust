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
    ({$($m:tt)*}$(($($p:tt)*))*) => (exp_call!((Exp::Closure(exp_match!($($m)*))), $(($($p)*))*));
    ($($x:ident):*$(($($p:tt)*))* -> $($rest:tt)*) => (Exp::Closure(vec![(pattern!($($x):*$(($($p)*))*), exp!($($rest)*))]));
    ($x:ident$([$($g:tt)*])*$(($($p:tt)*))+) => (exp_call!((exp!($x$([$($g)*])*)), $(($($p)*))+));
    (($($t:tt)*)$(($($p:tt)*))*) => (exp_call!((exp_tuple!(() $($t)*,)), $(($($p)*))*));
}

macro_rules! exp_call {
    (($($current:tt)*), ) => ($($current)*);
    (($($current:tt)*), ($($p:tt)*)$($rest:tt)*) => (exp_call!((Exp::Call(Box::new($($current)*), Box::new(exp_tuple!(() $($p)*,)))), $($rest)*));
}

macro_rules! exp_match {
    ($($($px:ident)*$(($($pp:tt)*))* => $($ex:ident)*$([$($eg:tt)*])*$(($($ep:tt)*))*),*) =>
        (vec![$((pattern!($($px)*$(($($pp)*))*), exp!($($ex)*$([$($eg)*])*$(($($ep)*))*))),*]);
}

macro_rules! exp_tuple {
    ((exp!($($e:tt)*), )$(,)* ) => (exp!($($e)*));
    (($($v:tt)*)$(,)* ) => (Exp::Tuple(vec!($($v)*)));
    (($($v:tt)*) $($({$($m:tt)*})*$($x:ident):*$([$($g:tt)*])*$(($($p:tt)*))*)->*, $($rest:tt)*) =>
        (exp_tuple!(($($v)* exp!($($({$($m)*})*$($x)*$([$($g)*])*$(($($p)*))*)->*), ) $($rest)*));
}

macro_rules! exp_id {
    ($x:ident) => (ExpID::Var($x.into(), vec![]));
    ($x:ident[$($g:tt)*]) => (ExpID::Var($x.into(), type_id_vec!(() $($g)*)));
    ($x:ident$([$($g:tt)*])*($($p:tt)*)) => (ExpID::Call(Box::new(exp_id!($x$([$($g)*])*)), Box::new(exp_id_tuple!(() $($p)*,))));
    (($($p:tt)*)) => (exp_id_tuple!(() $($p)*,));
}

macro_rules! exp_id_tuple {
    ((exp_id!($($e:tt)*)$(,)* )$(,)* ) => (exp_id!($($e)*));
    (($($v:tt)*)$(,)* ) => (ExpID::Tuple(vec!($($v)*)));
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
    (($($v:tt)*) $($x:ident):*$(($($p:tt)*))*$([$($g:tt)*])*, $($rest:tt)*) => (pattern_tuple!(($($v)* pattern!($($x):*$(($($p)*))*$([$($g)*])*), ) $($rest)*));
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
    ([]) => (Tree::default());
    (f) => (Tree::edge(TreeChar::Func));
    (t) => (Tree::edge(TreeChar::Tuple));
    ($f:tt) => (Tree::edge($f));
}

macro_rules! truth_ref {
    ($n:ident()) => (TruthRef::new(stringify!($n).to_owned(), vec![], None));
    ($n:ident($($p:tt)*)) => (TruthRef::new(stringify!($n).to_owned(), vec![], Some(exp_tuple!(() $($p)*,))));
}

macro_rules! proof {
    (match $($x:ident)*$([$($g:tt)*])*$(($($p:tt)*))* {$($m:tt)*}) =>
        (Proof::Match(exp!($($x)*$([$($g)*])*$(($($p)*))*), proof_match!($($m)*)));
    ($n:ident($($p:tt)*)$(~$fn:ident($($fp:tt)*)$([$($ft:tt)*])*)*$(.$($tn:ident($($tp:tt)*)$([$($tt:tt)*])*)~*)*) =>
        (Proof::Sequence(truth_ref!($n($($p)*)), proof_sequence!(() $(~$fn($($fp)*)$([$($ft)*])*)*$(.$($tn($($tp)*)$([$($tt)*])*)~*)*)));
}

macro_rules! proof_match {
    ($($($px:ident):*$([$($pg:tt)*])*$(($($pp:tt)*))* => $($($an:ident($($ap:tt)*)$([$($at:tt)*])*)~*).*$({$($b:tt)*})*),*) =>
        (vec![$((pattern!($($px):*$([$($pg)*])*$(($($pp)*))*), proof!($($($an($($ap)*)$([$($at)*])*)~*).*$({$($b)*})*))),*]);
}

macro_rules! proof_sequence {
    (($($v:tt)*)) => (vec![$($v)*]);
    (($($v:tt)*) .$n:ident($($p:tt)*)[$($t:tt)*]$($rest:tt)*) =>
        (proof_sequence!(($($v)* (Direction::Forwards, truth_ref!($n($($p)*)), tree!([$($t)*])), ) $($rest)*));
    (($($v:tt)*) ~$n:ident($($p:tt)*)[$($t:tt)*]$($rest:tt)*) =>
        (proof_sequence!(($($v)* (Direction::Backwards, truth_ref!($n($($p)*)), tree!([$($t)*])), ) $($rest)*));
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
        (Element::Func(stringify!($n).to_owned(), vec![], None, vec![(pattern_tuple!(()$($p)*,), exp!($($rest)*))]));
    (fn $n:ident[$($g:ident),*]($($p:tt)*) = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], None, vec![(pattern_tuple!(()$($p)*,), exp!($($rest)*))]));
    (fn $n:ident($($p:tt)*) -> $($t:ident)*$([$($tg:tt)*])*$(($($tp:tt)*))* = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![], Some(ttype!($($t)*$([$($tg)*])*$(($($tp)*))*)), vec![(pattern_tuple!(()$($p)*,), exp!($($rest)*))]));
    (fn $n:ident[$($g:ident),*]($($p:tt)*) -> $($t:ident)*$([$($tg:tt)*])*$(($($tp:tt)*))* = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], Some(ttype!($($t)*$([$($tg)*])*$(($($tp)*))*)), vec![(pattern_tuple!(()$($p)*,), exp!($($rest)*))]));
    (fn $n:ident {$($m:tt)*}) =>
        (Element::Func(stringify!($n).to_owned(), vec![], None, exp_match!($($m)*)));
    (fn $n:ident[$($g:ident),*] {$($m:tt)*}) =>
        (Element::Func(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], None, exp_match!($($m)*)));
    (fn $n:ident -> $($t:ident)*$([$($tg:tt)*])*$(($($tp:tt)*))* {$($m:tt)*}) =>
        (Element::Func(stringify!($n).to_owned(), vec![], Some(ttype!($($t)*$([$($tg)*])*$(($($tp)*))*)), exp_match!($($m)*)));
    (fn $n:ident[$($g:ident),*] -> $($t:ident)*$([$($tg:tt)*])*$(($($tp:tt)*))* {$($m:tt)*}) =>
        (Element::Func(stringify!($n).to_owned(), Some(ttype!($($t)*$([$($tg)*])*$(($($tp)*))*)), vec![$(stringify!($g).to_owned()),*], exp_match!($($m)*)));
    (proof $n:ident{ $($proof:tt)* }) =>
        (Element::Proof(stringify!($n).to_owned(), vec![], None, proof!($($proof)*)));
    (proof $n:ident[$($g:ident),*]{ $($proof:tt)* }) =>
        (Element::Proof(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], None, proof!($($proof)*)));
    (proof $n:ident($($p:tt)*){ $($proof:tt)* }) =>
        (Element::Proof(stringify!($n).to_owned(), vec![], Some(pattern_tuple!(()$($p)*,)), proof!($($proof)*)));
    (proof $n:ident[$($g:ident),*]($($p:tt)*){ $($proof:tt)* }) =>
        (Element::Proof(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], Some(pattern_tuple!(()$($p)*,)), proof!($($proof)*)));
}

macro_rules! element_gen {
    (($($v:tt)*), ) => (vec!($($v)*));
    (($($v:tt)*)  $x:ident, $($rest:tt)*) => (element_gen!(($($v)* (Variance::Invariant    , stringify!($x).to_owned())), ) $($rest)*);
    (($($v:tt)*) +$x:ident, $($rest:tt)*) => (element_gen!(($($v)* (Variance::Covariant    , stringify!($x).to_owned())), ) $($rest)*);
    (($($v:tt)*) -$x:ident, $($rest:tt)*) => (element_gen!(($($v)* (Variance::Contravariant, stringify!($x).to_owned())), ) $($rest)*);
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
    ($env:ident, let $n:ident$([$($g:tt)*])*$(: $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))*)* = $($({$($em:tt)*})*$($ex:ident):*$([$($eg:tt)*])*$(($($ep:tt)*))*)->*; $($rest:tt)*) => {
        element!(let $n$([$($g)*])*$(: $($t)*$([$($gs)*])*$(($($ps)*))*)* = $($({$($em)*})*$($ex)*$([$($eg)*])*$(($($ep)*))*)->*).define(&mut $env).unwrap();
        script!($env, $($rest)*);
    };
    ($env:ident, fn $n:ident$([$($g:tt)*])*($($p:tt)*)$(-> $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))*)* = $($({$($em:tt)*})*$($ex:ident):*$([$($eg:tt)*])*$(($($ep:tt)*))*)->*; $($rest:tt)*) => {
        element!(fn $n$([$($g)*])*($($p)*)$(-> $($t)*$([$($gs)*])*$(($($ps)*))*)* = $($({$($em)*})*$($ex)*$([$($eg)*])*$(($($ep)*))*)->*).define(&mut $env).unwrap();
        script!($env, $($rest)*);
    };
    ($env:ident, fn $n:ident$([$($g:tt)*])*$(-> $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))*)* {$($m:tt)*} $($rest:tt)*) => {
        element!(fn $n$([$($g)*])*$(-> $($t)*$([$($gs)*])*$(($($ps)*))*)* {$($m)*}).define(&mut $env).unwrap();
        script!($env, $($rest)*);
    };
    ($env:ident, proof $n:ident$([$($g:tt)*])*$(($($p:tt)*))*{$($proof:tt)*} $($rest:tt)*) => {
        element!(proof $n$([$($g)*])*$(($($p)*))*{$($proof)*}).define(&mut $env).unwrap();
        script!($env, $($rest)*);
    };
}

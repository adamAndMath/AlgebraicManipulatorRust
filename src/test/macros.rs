macro_rules! ttype {
    ($t:ident) => (Type::Gen(stringify!($t).to_owned(), vec!()));
    ($t:ident[$($g:tt)*]) => (Type::Gen(stringify!($t).to_owned(), ttype_vec!(() $($g)*,)));
    (($($p:tt)*)) => (Type::Tuple(ttype_vec!(() $($p)*,)));
}

macro_rules! ttype_vec {
    (($($v:tt)*)$(,)* ) => (vec!($($v)*));
    (($($v:tt)*) $t:ident ,$($rest:tt)*) => (ttype_vec!(($($v)* ttype!($t), ) $($rest)*));
    (($($v:tt)*) $t:ident[$($g:tt)*], $($rest:tt)*) => (ttype_vec!(($($v)* ttype!($t[$($g)*]), ) $($rest)*));
    (($($v:tt)*) ($($p:tt)*), $($rest:tt)*) => (ttype_vec!(($($v)* ttype!(($($p)*)), ) $($rest)*));
}

macro_rules! type_id {
    ($t:ident) => (TypeID::Gen($t, vec!()));
    ($t:ident[$($g:tt)*]) => (TypeID::Gen($t, type_id_gen!(() $($g)*,)));
    (($($p:tt)*)) => (type_id_tuple!(() $($p)*,));
}

macro_rules! type_id_gen {
    (($($v:tt)*)$(,)* ) => (vec!($($v)*));
    (($($v:tt)*) $t:ident, $($rest:tt)*) => (type_id_gen!(($($v)* (Variance::Invariant, type_id!($t)), ) $($rest)*));
    (($($v:tt)*) +$t:ident, $($rest:tt)*) => (type_id_gen!(($($v)* (Variance::Covariant, type_id!($t)), ) $($rest)*));
    (($($v:tt)*) -$t:ident, $($rest:tt)*) => (type_id_gen!(($($v)* (Variance::Contravariant, type_id!($t)), ) $($rest)*));
    (($($v:tt)*) $t:ident[$($g:tt)*], $($rest:tt)*) => (type_id_gen!(($($v)* (Variance::Invariant, type_id!($t[$($g)*]))) $($rest)*));
    (($($v:tt)*) +$t:ident[$($g:tt)*], $($rest:tt)*) => (type_id_gen!(($($v)* (Variance::Covariant, type_id!($t[$($g)*]))) $($rest)*));
    (($($v:tt)*) -$t:ident[$($g:tt)*], $($rest:tt)*) => (type_id_gen!(($($v)* (Variance::Contravariant, type_id!($t[$($g)*]))) $($rest)*));
    (($($v:tt)*) ($($p:tt)*), $($rest:tt)*) => (type_id_gen!(($($v)* (Variance::Invariant, type_id!(($($p),*)))) $($rest)*));
    (($($v:tt)*) +($($p:tt)*), $($rest:tt)*) => (type_id_gen!(($($v)* (Variance::Covariant, type_id!(($($p),*)))) $($rest)*));
    (($($v:tt)*) -($($p:tt)*), $($rest:tt)*) => (type_id_gen!(($($v)* (Variance::Contravariant, type_id!(($($p),*)))) $($rest)*));
}

macro_rules! type_id_tuple {
    ((type_id!($($e:tt)*))$(,)* ) => (type_id!($($e)*));
    (($($v:tt)*)$(,)* ) => (TypeID::Tuple(vec!($($v)*)));
    (($($v:tt)*) $t:ident, $($rest:tt)*) => (type_id_tuple!(($($v)* type_id!($t), ) $($rest)*));
    (($($v:tt)*) $t:ident[$($g:tt)*], $($rest:tt)*) => (type_id_tuple!(($($v)* type_id!($t[$($g)*])) $($rest)*));
    (($($v:tt)*) ($($p:tt)*), $($rest:tt)*) => (type_id_tuple!(($($v)* type_id!(($($p),*))) $($rest)*));
}

macro_rules! exp {
    ($x:ident) => (Exp::Var(stringify!($x).to_owned()));
    ($x:ident($($p:tt)*)) => (Exp::Call(Box::new(exp!($x)), Box::new(exp_tuple!(() $($p)*,))));
    (($($p:tt)*)) => (exp_tuple!(() $($p)*,));
}

macro_rules! exp_tuple {
    ((exp!($($e:tt)*))$(,)* ) => (exp!($($e)*));
    (($($v:tt)*)$(,)* ) => (Exp::Tuple(vec!($($v)*)));
    (($($v:tt)*) $x:ident$(($($p:tt)*))*, $($rest:tt)*) => (exp_tuple!(($($v)* exp!($x$(($($p)*))*)) $($rest)*));
    (($($v:tt)*) $(($($p:tt)*))+, $($rest:tt)*) => (exp_tuple!(($($v)* exp!($(($($p)*))*)) $($rest)*));
}

macro_rules! exp_id {
    ($x:ident) => (ExpID::Var($x.into()));
    ($x:ident($($p:tt)*)) => (ExpID::Call(Box::new(exp_id!($x)), Box::new(exp_id_tuple!(() $($p)*,))));
    (($($p:tt)*)) => (exp_id_tuple!(() $($p)*,));
}

macro_rules! exp_id_tuple {
    ((exp_id!($($e:tt)*))$(,)* ) => (exp_id!($($e)*));
    (($($v:tt)*), ) => (ExpID::Tuple(vec!($($v)*)));
    (($($v:tt)*) $x:ident$(($($p:tt)*))*, $($rest:tt)*) => (exp_id_tuple!(($($v)* exp_id!($x$(($($p)*))*)) $($rest)*));
    (($($v:tt)*) $(($($p:tt)*))+, $($rest:tt)*) => (exp_id_tuple!(($($v)* exp_id!($(($($p)*))*)) $($rest)*));
}

macro_rules! element {
    (struct $n:ident) => (Element::Struct(stringify!($n).to_owned(), vec!()));
    (struct $n:ident($($v:tt)*)) => (Element::Struct(stringify!($n).to_owned(), ttype_vec!(() $($v)*,)));
    (enum $n:ident { $($v:tt)* }) => (Element::Enum(stringify!($n).to_owned(), enum_variants!(() $($v)*,)));
    (let $n:ident = $($e:tt)*) => (Element::Let(stringify!($n).to_owned(), None, exp!($($e)*)));
    (let $n:ident: $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))* = $($e:tt)*) => (Element::Let(stringify!($n).to_owned(), Some(ttype!($($t)*$([$($gs)*])*$(($($ps)*))*)), exp!($($e)*)));
}

macro_rules! enum_variants {
    (($($v:tt)*)$(,)* ) => (vec!($($v)*));
    (($($v:tt)*) $n:ident, $($rest:tt)*) => (enum_variants!(($($v)* (stringify!($n).to_owned(), vec!()),) $($rest)*));
    (($($v:tt)*) $n:ident($($t:tt)*), $($rest:tt)*) => (enum_variants!(($($v)* (stringify!($n).to_owned(), ttype_vec!(() $($t)*,)),) $($rest)*));
}

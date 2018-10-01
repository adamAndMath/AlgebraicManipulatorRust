macro_rules! element {
    (struct $n:ident) => (Element::Struct(stringify!($n).to_owned(), vec![], vec![]));
    (struct $n:ident[$(g:tt)*]) =>
        (Element::Struct(stringify!($n).to_owned(), element_gen!(() $($g)*,), vec![]));
    (struct $n:ident($($v:tt)*)) =>
        (Element::Struct(stringify!($n).to_owned(), vec![], ttype_vec!($($v)*)));
    (struct $n:ident[$($g:tt)*]($($v:tt)*)) =>
        (Element::Struct(stringify!($n).to_owned(), element_gen!(() $($g)*,), ttype_vec!($($v)*)));
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
        (Element::Func(stringify!($n).to_owned(), vec![], None, vec![(pattern_tuple!($($p)*), exp!($($rest)*))]));
    (fn $n:ident[$($g:ident),*]($($p:tt)*) = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], None, vec![(pattern_tuple!($($p)*), exp!($($rest)*))]));
    (fn $n:ident($($p:tt)*) -> $($t:ident)*$([$($tg:tt)*])*$(($($tp:tt)*))* = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![], Some(ttype!($($t)*$([$($tg)*])*$(($($tp)*))*)), vec![(pattern_tuple!($($p)*), exp!($($rest)*))]));
    (fn $n:ident[$($g:ident),*]($($p:tt)*) -> $($t:ident)*$([$($tg:tt)*])*$(($($tp:tt)*))* = $($rest:tt)*) =>
        (Element::Func(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], Some(ttype!($($t)*$([$($tg)*])*$(($($tp)*))*)), vec![(pattern_tuple!($($p)*), exp!($($rest)*))]));
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
        (Element::Proof(stringify!($n).to_owned(), vec![], Some(pattern_tuple!($($p)*)), proof!($($proof)*)));
    (proof $n:ident[$($g:ident),*]($($p:tt)*){ $($proof:tt)* }) =>
        (Element::Proof(stringify!($n).to_owned(), vec![$(stringify!($g).to_owned()),*], Some(pattern_tuple!($($p)*)), proof!($($proof)*)));
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
    (($($v:tt)*) $n:ident($($t:tt)*), $($rest:tt)*) => (enum_variants!(($($v)* (stringify!($n).to_owned(), ttype_vec!($($t)*)),) $($rest)*));
}

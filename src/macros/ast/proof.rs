macro_rules! truth_ref {
    ($n:ident()) => (TruthRef::new(stringify!($n).to_owned(), vec![], None));
    ($n:ident($($p:tt)*)) => (TruthRef::new(stringify!($n).to_owned(), vec![], Some(exp_tuple!($($p)*))));
    ($n:ident[$($g:tt)*]($($p:tt)*)) => (TruthRef::new(stringify!($n).to_owned(), ttype_vec!($($g)*), Some(exp_tuple!($($p)*))));
}

macro_rules! proof {
    (match $($x:ident)*$([$($g:tt)*])*$(($($p:tt)*))* {$($m:tt)*}) =>
        (Proof::Match(exp!($($x)*$([$($g)*])*$(($($p)*))*), proof_match!($($m)*)));
    ($n:ident$([$($g:tt)*])*($($p:tt)*)$(~$fn:ident($($fp:tt)*)$([$($ft:tt)*])*)*$(.$($tn:ident($($tp:tt)*)$([$($tt:tt)*])*)~*)*) =>
        (Proof::Sequence(truth_ref!($n$([$($g)*])*($($p)*)), proof_sequence!(() $(~$fn($($fp)*)$([$($ft)*])*)*$(.$($tn($($tp)*)$([$($tt)*])*)~*)*)));
}

macro_rules! proof_match {
    ($($($px:ident):*$([$($pg:tt)*])*$(($($pp:tt)*))* => $($($an:ident$([$($ag:tt)*])*($($ap:tt)*)$([$($at:tt)*])*)~*).*$({$($b:tt)*})*),*) =>
        (vec![$((pattern!($($px):*$([$($pg)*])*$(($($pp)*))*), proof!($($($an$([$($ag)*])*($($ap)*)$([$($at)*])*)~*).*$({$($b)*})*))),*]);
}

macro_rules! proof_sequence {
    (($($v:tt)*)) => (vec![$($v)*]);
    (($($v:tt)*) .$n:ident$([$($g:tt)*])*($($p:tt)*)[$($t:tt)*]$($rest:tt)*) =>
        (proof_sequence!(($($v)* (Direction::Forwards, truth_ref!($n$([$($g)*])*($($p)*)), tree!([$($t)*])), ) $($rest)*));
    (($($v:tt)*) ~$n:ident$([$($g:tt)*])*($($p:tt)*)[$($t:tt)*]$($rest:tt)*) =>
        (proof_sequence!(($($v)* (Direction::Backwards, truth_ref!($n$([$($g)*])*($($p)*)), tree!([$($t)*])), ) $($rest)*));
}

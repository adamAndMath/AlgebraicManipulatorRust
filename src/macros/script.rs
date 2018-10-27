macro_rules! script {
    ($space:ident, $env:ident, $($rest:tt)*) => (script!((), $space, $env, $($rest)*));
    (($($v:tt)*), $space:ident, $env:ident, ) => (vec![$($v)*].into_iter().map(|e|e.to_id(&mut $space)).collect::<Result<Vec<_>,_>>().unwrap().into_iter().flatten().map(|e|e.define(&mut $env)).collect::<Result<(),_>>().unwrap());
    (($($v:tt)*), $space:ident, $env:ident, struct $n:ident$([$($g:tt)*])*$(($($p:tt)*))*; $($rest:tt)*) => (
        script!(($($v)* element!(struct $n$([$($g)*])*$(($($p)*))*),), $space, $env, $($rest)*);
    );
    (($($v:tt)*), $space:ident, $env:ident, enum $n:ident$([$($g:tt)*])*{$($vs:tt)*} $($rest:tt)*) => (
        script!(($($v)* element!(enum $n$([$($g)*])*{$($vs)*}),), $space, $env, $($rest)*)
    );
    (($($v:tt)*), $space:ident, $env:ident, let $n:ident$([$($g:tt)*])*$(: $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))*)* = $($({$($em:tt)*})*$($ex:ident):*$(::$ey:ident)*$([$($eg:tt)*])*$(($($ep:tt)*))*)->*; $($rest:tt)*) => (
        script!(($($v)* element!(let $n$([$($g)*])*$(: $($t)*$([$($gs)*])*$(($($ps)*))*)* = $($({$($em)*})*$($ex)*$(::$ey)*$([$($eg)*])*$(($($ep)*))*)->*),), $space, $env, $($rest)*)
    );
    (($($v:tt)*), $space:ident, $env:ident, fn $n:ident$([$($g:tt)*])*($($p:tt)*)$(-> $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))*)* = $($({$($em:tt)*})*$($ex:ident):*$(::$ey:ident)*$([$($eg:tt)*])*$(($($ep:tt)*))*)->*; $($rest:tt)*) => (
        script!(($($v)* element!(fn $n$([$($g)*])*($($p)*)$(-> $($t)*$([$($gs)*])*$(($($ps)*))*)* = $($({$($em)*})*$($ex)*$(::$ey)*$([$($eg)*])*$(($($ep)*))*)->*),), $space, $env, $($rest)*)
    );
    (($($v:tt)*), $space:ident, $env:ident, fn $n:ident$([$($g:tt)*])*$(-> $($t:ident)*$([$($gs:tt)*])*$(($($ps:tt)*))*)* {$($m:tt)*} $($rest:tt)*) => (
        script!(($($v)* element!(fn $n$([$($g)*])*$(-> $($t)*$([$($gs)*])*$(($($ps)*))*)* {$($m)*}),), $space, $env, $($rest)*)
    );
    (($($v:tt)*), $space:ident, $env:ident, proof $n:ident$([$($g:tt)*])*$(($($p:tt)*))*{$($proof:tt)*} $($rest:tt)*) => (
        script!(($($v)* element!(proof $n$([$($g)*])*$(($($p)*))*{$($proof)*}),), $space, $env, $($rest)*)
    );
}

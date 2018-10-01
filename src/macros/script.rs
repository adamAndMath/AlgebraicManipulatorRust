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

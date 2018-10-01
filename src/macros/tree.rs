macro_rules! tree {
    ($f:tt$(,$t:tt)+) => (tree!($f)$(+tree!($t))*);
    ([$($f:tt),+$(|$($t:tt),+)*]) => (tree!($($f),+)$(*tree!($($t),+))*);
    ([]) => (Tree::default());
    (f) => (Tree::edge(TreeChar::Func));
    (t) => (Tree::edge(TreeChar::Tuple));
    ($f:tt) => (Tree::edge($f));
}

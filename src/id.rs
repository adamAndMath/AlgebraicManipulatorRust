#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocalID {
    Global(usize),
    Local(usize),
}
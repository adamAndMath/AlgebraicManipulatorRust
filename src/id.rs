#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ID {
    Global(usize),
    Local(usize),
}
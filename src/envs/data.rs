use super::{ ExpVal, TypeVal, TruthVal };

pub struct EnvsData {
    pub exps: Vec<ExpVal>,
    pub types: Vec<TypeVal>,
    pub truths: Vec<TruthVal>,
}

impl EnvsData {
    #[cfg(test)]
    pub fn lens(&self) -> (usize, usize, usize) {
        (self.exps.len(), self.types.len(), self.truths.len())
    }
}
use env::EnvData;
use super::{ ExpVal, TypeVal, TruthVal };

pub struct EnvsData {
    pub exps: EnvData<ExpVal>,
    pub types: EnvData<TypeVal>,
    pub truths: EnvData<TruthVal>,
}

impl EnvsData {
    pub fn new(exps: Vec<ExpVal>, types: Vec<TypeVal>, truths: Vec<TruthVal>) -> Self {
        EnvsData {
            exps: EnvData::new(exps),
            types: EnvData::new(types),
            truths: EnvData::new(truths),
        }
    }

    #[cfg(test)]
    pub fn lens(&self) -> (usize, usize, usize) {
        (self.exps.len(), self.types.len(), self.truths.len())
    }
}
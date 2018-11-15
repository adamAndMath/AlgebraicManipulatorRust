use env::PushID;
use id::Exp;

#[derive(Debug)]
pub struct MatchEnv<'a> {
    data: Vec<(Exp, Exp)>,
    parent: Option<&'a MatchEnv<'a>>,
}

impl<'a> MatchEnv<'a> {
    pub fn new() -> Self {
        MatchEnv { data: vec![], parent: None }
    }

    pub fn scope<'b>(&'b self, v: Vec<(Exp, Exp)>) -> MatchEnv<'b> where 'a: 'b {
        MatchEnv { data: v, parent: Some(&self) }
    }

    pub fn add(&mut self, k: Exp, v: Exp) {
        self.data.push((k, v));
    }

    pub fn get(&self, k: &Exp) -> Option<Exp> {
        self.data.iter().filter(|(i,_)|i==k).map(|(_,v)|v.clone()).next().or_else(||self.parent.and_then(|p|k.pop_id(1).and_then(|k|p.get(&k).map(|v|v.push_id(1)))))
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
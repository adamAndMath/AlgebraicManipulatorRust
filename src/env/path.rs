use pest::Span;

#[derive(Debug, Eq, Clone, Hash)]
pub struct Path<'f>(Span<'f>, Vec<&'f str>);

impl<'f> Path<'f> {
    pub fn new(span: Span<'f>, v: Vec<&'f str>) -> Self {
        Path(span, v)
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.1.iter().map(|s|*s)
    }

    pub fn name(&self) -> &'f str {
        self.1[self.1.len() - 1]
    }

    pub fn as_span(&self) -> Span<'f> {
        self.0.clone()
    }
}

impl<'f> PartialEq for Path<'f> {
    fn eq(&self, rhs: &Path<'f>) -> bool {
        self.1 == rhs.1
    }
}

impl<'f> From<(Span<'f>)> for Path<'f> {
    fn from(span: Span<'f>) -> Self {
        let s = span.as_str();
        Path(span, vec![s])
    }
}

impl<'f> AsRef<[&'f str]> for Path<'f> {
    fn as_ref(&self) -> &[&'f str] {
        &self.1
    }
}
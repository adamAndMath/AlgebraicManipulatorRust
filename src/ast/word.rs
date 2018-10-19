use pest::Span;

#[derive(Debug, Clone, Eq)]
pub struct Word<'f> {
    span: Span<'f>,
}

impl<'f> Word<'f> {
    pub fn new(span: Span<'f>) -> Self {
        Word { span }
    }

    pub fn as_span(&self) -> Span<'f> {
        self.span.clone()
    }
}

impl<'f> PartialEq for Word<'f> {
    fn eq(&self, rhs: &Self) -> bool {
        self.span.as_str() == rhs.span.as_str()
    }
}

impl<'f> AsRef<str> for Word<'f> {
    fn as_ref(&self) -> &str {
        self.span.as_str()
    }
}

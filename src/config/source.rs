use std::fmt::Display;

use miette::{MietteSpanContents, NamedSource, SourceCode};

#[derive(Debug, Clone)]
pub struct Source {
    pub path: String,
    pub raw: String,
}

impl Source {
    pub fn new(name: String, raw: String) -> Self {
        Self { path: name, raw }
    }

    pub fn load(path: String) -> Result<Self, std::io::Error> {
        let raw = std::fs::read_to_string(&path)?;
        Ok(Self { raw, path })
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl SourceCode for Source {
    fn read_span<'a>(
        &'a self,
        span: &miette::SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let contents = self
            .raw
            .read_span(span, context_lines_before, context_lines_after)?;
        dbg!(&self.path);
        Ok(Box::new(MietteSpanContents::new_named(
            self.path.clone(),
            contents.data(),
            *contents.span(),
            contents.line(),
            contents.column(),
            contents.line_count(),
        )))
    }
}

impl From<&Source> for NamedSource<String> {
    fn from(source: &Source) -> Self {
        let name = source.path.clone();
        let input = source.raw.clone();
        NamedSource::new(name, input)
    }
}

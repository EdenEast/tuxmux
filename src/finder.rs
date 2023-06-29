use std::{fmt::Display, io::Cursor};

use itertools::Itertools;
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

use crate::config::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Global,
    Local,
}

#[derive(Debug, Default)]
pub struct FinderOptions<'a> {
    pub mode: Mode,
    pub query: Option<&'a str>,
    pub exact: bool,
    pub multi: bool,
}

pub fn find<'a, S, I>(iter: I, opts: FinderOptions) -> Vec<String>
where
    S: AsRef<str> + Display,
    I: Iterator<Item = S>,
{
    inner(iter, opts).unwrap_or_default()
}

fn inner<'a, S, I>(mut iter: I, opts: FinderOptions) -> Option<Vec<String>>
where
    S: AsRef<str> + Display,
    I: Iterator<Item = S>,
{
    let height = match opts.mode {
        Mode::Full => "100%".to_string(),
        Mode::Inline(h) => format!("{}%", h),
    };

    let skim_options = SkimOptionsBuilder::default()
        .exact(opts.exact)
        .multi(opts.multi)
        .query(opts.query)
        .reverse(true)
        .height(Some(height.as_str()))
        .build()
        .ok()?;

    let input = iter.join("\n");
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));
    let output = Skim::run_with(&skim_options, Some(items))?;

    if output.is_abort {
        return None;
    }

    Some(
        output
            .selected_items
            .iter()
            .map(|i| i.output().to_string())
            .collect_vec(),
    )
}

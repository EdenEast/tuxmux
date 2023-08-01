use dialoguer::{
    console::{Style, Term},
    theme::ColorfulTheme,
    FuzzySelect, MultiSelect, Select,
};
use miette::IntoDiagnostic;

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
}

pub fn find<T: ToString + Clone>(items: &[T], opts: FinderOptions) -> miette::Result<Option<T>> {
    let mut theme = ColorfulTheme::default();
    theme.fuzzy_match_highlight_style = Style::new().for_stderr().red();
    let (term, height) = terminal_and_height(opts.mode);

    let selection = if opts.exact {
        Select::with_theme(&theme)
            .default(0)
            .items(items)
            .max_length(height)
            .interact_on_opt(&term)
            .into_diagnostic()?
    } else {
        FuzzySelect::with_theme(&theme)
            .default(0)
            .items(&items)
            .with_initial_text(opts.query.unwrap_or_default())
            .max_length(height)
            .interact_on_opt(&term)
            .into_diagnostic()?
    };

    Ok(selection.map(|i| items[i].clone()))
}

pub fn select_multi<T: ToString + Clone>(
    items: &[T],
    opts: FinderOptions,
) -> miette::Result<Option<Vec<T>>> {
    let (term, height) = terminal_and_height(opts.mode);
    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(items)
        .max_length(height)
        .interact_on_opt(&term)
        .into_diagnostic()?;

    Ok(selected.and_then(|idxs| {
        let mut r = Vec::new();
        for i in idxs {
            r.push(items[i].clone());
        }
        Some(r)
    }))
}

fn terminal_and_height(mode: Mode) -> (Term, usize) {
    let term = Term::stderr();
    let (rows, _) = term.size();
    let h = match mode {
        crate::config::Mode::Full => rows as usize,
        crate::config::Mode::Lines(lines) => lines as usize,
        crate::config::Mode::Percentage(percentage) => (rows as f32 * percentage) as usize,
    };

    (term, h)
}

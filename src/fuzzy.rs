use std::io::Cursor;

use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};

use crate::util::intersperse;

pub fn fuzzy_select_one<'a, I>(iter: I, query: Option<&str>, exact:bool) -> Option<String>
where
    I: Iterator<Item = &'a str>,
{
    let skim_options = SkimOptionsBuilder::default()
        .exit0(true)
        .select1(true)
        .height(Some("40"))
        .reverse(true)
        .exact(exact)
        .query(query)
        .build()
        .unwrap();

    let input: String = intersperse(iter, "\n").collect();
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));
    let (is_abort, selected_items) = Skim::run_with(&skim_options, Some(items))
        .map(|out| (out.is_abort, out.selected_items))
        .unwrap_or_else(|| (true, Vec::new()));

    match (selected_items.first(), is_abort) {
        (Some(item), false) => Some(item.output().into_owned()),
        _ => None,
    }
}

pub fn fuzzy_select_multi<'a, I>(iter: I, query: Option<&str>, exact: bool) -> Vec<String>
where
    I: Iterator<Item = &'a str>,
{
    let skim_options = SkimOptionsBuilder::default()
        .exit0(true)
        .select1(true)
        .height(Some("40"))
        .reverse(true)
        .exact(exact)
        .query(query)
        .multi(true)
        .build()
        .unwrap();

    let input: String = intersperse(iter, "\n").collect();
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));
    let (is_abort, selected_items) = Skim::run_with(&skim_options, Some(items))
        .map(|out| (out.is_abort, out.selected_items))
        .unwrap_or_else(|| (true, Vec::new()));

    if is_abort {
        return vec![];
    }

    selected_items
        .iter()
        .map(|i| i.output().into_owned())
        .collect()
}

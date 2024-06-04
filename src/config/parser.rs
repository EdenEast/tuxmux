use itertools::Itertools;
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};

use super::{error::ParseError, source::Source, Config};

#[derive(Debug)]
pub struct Parser {
    src: Source,
}

fn type_from_value(value: &KdlValue) -> &'static str {
    match value {
        KdlValue::RawString(_) | KdlValue::String(_) => "string",
        KdlValue::Null => "null",
        KdlValue::Bool(_) => "boolean",
        KdlValue::Base10Float(_) => "float",
        _ => "number",
    }
}

fn get_dash_values<'a>(doc: &'a KdlDocument, name: &'static str) -> Vec<&'a KdlEntry> {
    doc.get(name)
        .and_then(|n| n.children())
        .map(|doc| doc.nodes())
        .unwrap_or_default()
        .iter()
        .filter(|e| e.name().value() == "-")
        .filter_map(|n| n.get(0))
        .collect_vec()
}

impl Parser {
    pub fn new(source: Source) -> Self {
        Self { src: source }
    }

    pub fn parse(self) -> Result<Config, ParseError> {
        self.parse_with_default(None)
    }

    pub fn parse_with_default(self, default: Option<Config>) -> Result<Config, ParseError> {
        self.inner_parse(default.unwrap_or_default())
    }

    fn inner_parse(self, mut config: Config) -> Result<Config, ParseError> {
        let doc: KdlDocument = self.src.raw.parse()?;

        for node in doc.nodes() {
            match node.name().value() {
                "paths" => {
                    if let Some(doc) = node.children() {
                        if let Some(workspace_node) = doc.get("workspace") {
                            let default = self.get_default_optional(workspace_node)?;
                            let mut workspaces =
                                self.try_get_dash_values_as_valid_paths(doc, "workspace")?;

                            if default {
                                config.search.workspace.append(&mut workspaces);
                            } else {
                                config.search.workspace = workspaces;
                            }
                        }

                        if let Some(single_node) = doc.get("single") {
                            let default = self.get_default_optional(single_node)?;
                            let mut singles =
                                self.try_get_dash_values_as_valid_paths(doc, "single")?;

                            if default {
                                config.search.single.append(&mut singles);
                            } else {
                                config.search.single = singles;
                            }
                        }
                    }
                }
                "exclude_path" => {
                    let default = self.get_default_optional(node)?;
                    let paths = self.try_get_dash_values_as_string(&doc, "exclude_path")?;

                    if default {
                        config.exclude_path.extend(paths);
                    } else {
                        config.exclude_path = paths.into_iter().collect();
                    }
                }
                "depth" => {
                    config.depth = usize::try_from(self.first_entry_as_i64(node)?).unwrap_or(0);
                }
                "default_worktree" => {
                    config.default_worktree = self.first_entry_as_bool(node).unwrap_or(false);
                }
                option => {
                    return Err(ParseError::UnknownConfigurationOption(
                        option.to_owned(),
                        self.src.clone(),
                        *node.name().span(),
                    ));
                }
            }
        }

        Ok(config)
    }

    fn first_entry<'a>(&'a self, node: &'a KdlNode) -> Result<&'a KdlEntry, ParseError> {
        node.entries()
            .iter()
            .next()
            .ok_or(ParseError::MissingValue(self.src.clone(), *node.span()))
    }

    // fn first_entry_as_string<'a>(&'a self, node: &'a KdlNode) -> Result<&'a str, ParseError> {
    //     self.first_entry(node).and_then(|entry| {
    //         entry.value().as_string().ok_or(ParseError::TypeMismatch(
    //             "string",
    //             type_from_value(entry.value()),
    //             self.src.clone(),
    //             *entry.span(),
    //         ))
    //     })
    // }

    fn first_entry_as_i64<'a>(&'a self, node: &'a KdlNode) -> Result<i64, ParseError> {
        self.first_entry(node).and_then(|entry| {
            entry.value().as_i64().ok_or(ParseError::TypeMismatch(
                "number",
                type_from_value(entry.value()),
                self.src.clone(),
                *entry.span(),
            ))
        })
    }

    fn first_entry_as_bool<'a>(&'a self, node: &'a KdlNode) -> Result<bool, ParseError> {
        self.first_entry(node).and_then(|entry| {
            entry.value().as_bool().ok_or(ParseError::TypeMismatch(
                "bool",
                type_from_value(entry.value()),
                self.src.clone(),
                *entry.span(),
            ))
        })
    }

    fn get_default_optional(&self, node: &KdlNode) -> Result<bool, ParseError> {
        match node.get("default") {
            Some(value) => value.value().as_bool().ok_or(ParseError::TypeMismatch(
                "boolean",
                type_from_value(value.value()),
                self.src.clone(),
                *value.span(),
            )),
            None => Ok(true),
        }
    }

    fn try_get_dash_values_as_string(
        &self,
        doc: &KdlDocument,
        name: &'static str,
    ) -> Result<Vec<String>, ParseError> {
        get_dash_values(doc, name)
            .into_iter()
            .map(|entry| {
                entry
                    .value()
                    .as_string()
                    .ok_or(ParseError::TypeMismatch(
                        "string",
                        type_from_value(entry.value()),
                        self.src.clone(),
                        *entry.span(),
                    ))
                    .map(|s| s.to_string())
            })
            .try_collect()
    }

    fn try_get_dash_values_as_valid_paths(
        &self,
        doc: &KdlDocument,
        name: &'static str,
    ) -> Result<Vec<String>, ParseError> {
        get_dash_values(doc, name)
            .into_iter()
            .map(|entry| {
                entry
                    .value()
                    .as_string()
                    .ok_or(ParseError::TypeMismatch(
                        "string",
                        type_from_value(entry.value()),
                        self.src.clone(),
                        *entry.span(),
                    ))
                    .map(|s| shellexpand::tilde(s).to_string())
            })
            .try_collect()
    }
}

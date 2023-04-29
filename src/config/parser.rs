use indexmap::IndexMap;
use itertools::Itertools;
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};

use crate::finder::FinderChoice;

use super::{error::ParseError, source::Source, Config, WorkspaceDefinition};

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

        if let Some(workspace_node) = doc.get("workspaces") {
            let default = self.get_default_optional(workspace_node)?;
            if let Some(child) = workspace_node.children() {
                let definitions: IndexMap<String, WorkspaceDefinition> = child
                    .nodes()
                    .iter()
                    .map(|node| self.workspace_definition(node))
                    .map(|r| r.map(|d| (d.name.clone(), d)))
                    .try_collect()?;

                if default {
                    config.definitions.extend(definitions.into_iter());
                } else {
                    config.definitions = definitions;
                }
            }
        }

        if let Some(exclude_node) = doc.get("exclude_path") {
            let default = self.get_default_optional(exclude_node)?;
            let paths = self.try_get_dash_values_as_string(&doc, "exclude_path")?;

            if default {
                config.exclude_path.extend(paths.into_iter());
            } else {
                config.exclude_path = paths.into_iter().collect();
            }
        }

        if let Some(doc) = doc.get("paths").and_then(|node| node.children()) {
            if let Some(workspace_node) = doc.get("workspace") {
                let default = self.get_default_optional(workspace_node)?;
                let mut workspaces = self.try_get_dash_values_as_valid_paths(doc, "workspace")?;

                if default {
                    config.search.workspace.append(&mut workspaces);
                } else {
                    config.search.workspace = workspaces;
                }
            }

            if let Some(single_node) = doc.get("single") {
                let default = self.get_default_optional(single_node)?;
                let mut singles = self.try_get_dash_values_as_valid_paths(doc, "single")?;

                if default {
                    config.search.single.append(&mut singles);
                } else {
                    config.search.single = singles;
                }
            }
        }

        if let Some(node) = doc.get("depth") {
            config.depth = usize::try_from(self.first_entry_as_i64(node)?).unwrap_or(0);
        }

        if let Some(node) = doc.get("height") {
            config.height = usize::try_from(self.first_entry_as_i64(node)?).unwrap_or(0);
        }

        if let Some(node) = doc.get("finder") {
            let value = match self.first_entry_as_string(node)? {
                "fzf" => Ok(FinderChoice::Fzf),
                "skim" => Ok(FinderChoice::Skim),
                _ => Err(ParseError::InvalidFinder(self.src.clone(), *node.span())),
            }?;
            config.finder = value;
        }

        Ok(config)
    }

    fn first_entry<'a>(&'a self, node: &'a KdlNode) -> Result<&'a KdlEntry, ParseError> {
        node.entries()
            .iter()
            .next()
            .ok_or(ParseError::MissingValue(self.src.clone(), *node.span()))
    }

    fn first_entry_as_string<'a>(&'a self, node: &'a KdlNode) -> Result<&'a str, ParseError> {
        self.first_entry(node).and_then(|entry| {
            entry.value().as_string().ok_or(ParseError::TypeMismatch(
                "string",
                type_from_value(entry.value()),
                self.src.clone(),
                *entry.span(),
            ))
        })
    }

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

    fn workspace_definition(&self, node: &KdlNode) -> Result<WorkspaceDefinition, ParseError> {
        if node.name().value() != "workspace" {
            return Err(ParseError::NodeMismatch(
                "workspace",
                node.name().value().to_string(),
                self.src.clone(),
                *node.span(),
            ));
        }

        let name_node = node
            .entries()
            .first()
            .and_then(|entry| match entry.name() {
                Some(_) => None,
                None => Some(entry),
            })
            .ok_or(ParseError::MissingPositionalEntry(
                "name",
                self.src.clone(),
                *node.span(),
            ))?;

        let name = name_node
            .value()
            .as_string()
            .ok_or(ParseError::TypeMismatch(
                "string",
                type_from_value(name_node.value()),
                self.src.clone(),
                *name_node.span(),
            ))?
            .to_string();

        let layout = match node.get("layout") {
            Some(entry) => Some(
                entry
                    .value()
                    .as_string()
                    .ok_or(ParseError::TypeMismatch(
                        "string",
                        type_from_value(entry.value()),
                        self.src.clone(),
                        *node.span(),
                    ))?
                    .to_string(),
            ),
            None => None,
        };

        let child = node.children().ok_or(ParseError::MissingNode(
            "files",
            self.src.clone(),
            *node.span(),
        ))?;

        if child.get("files").is_none() {
            return Err(ParseError::MissingNode(
                "files",
                self.src.clone(),
                *node.span(),
            ));
        }

        let files: Vec<String> = child
            .get_dash_vals("files")
            .into_iter()
            .map(|f| {
                f.as_string()
                    .map(ToString::to_string)
                    .ok_or(ParseError::TypeMismatch(
                        "string",
                        type_from_value(f),
                        self.src.clone(),
                        *node.span(),
                    ))
            })
            .try_collect()?;

        Ok(WorkspaceDefinition {
            name,
            files,
            layout,
        })
    }
}

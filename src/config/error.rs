use kdl::KdlError;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use super::source::Source;

#[derive(Debug, Error, Diagnostic)]
#[error("{kind}")]
pub struct ConfigError {
    #[source_code]
    pub src: Source,
    pub kind: ParseError,
}

#[derive(Debug, Error, Diagnostic)]
pub enum ParseError {
    #[error("Node mismatch")]
    #[diagnostic(code("tm::node_mismatch"))]
    NodeMismatch(
        /// The node expected
        &'static str,
        /// The node given
        String,
        #[source_code] Source,
        #[label("expected {0} found {1}")] SourceSpan,
    ),

    #[error("Type mismatch")]
    #[diagnostic(code("tm::type_mismatch"))]
    TypeMismatch(
        /// The type expected
        &'static str,
        /// The value given
        &'static str,
        #[source_code] Source,
        #[label("expected type {0} found {1}")] SourceSpan,
    ),

    #[error("Missing positional entry")]
    #[diagnostic(code("tm::missing_positional_entry"))]
    MissingPositionalEntry(
        /// Type of the missing positional
        &'static str,
        #[source_code] Source,
        #[label("expected entry {0}")] SourceSpan,
    ),

    #[error("Missing node")]
    #[diagnostic(code("tm::missing_node"))]
    MissingNode(
        /// name of missing node
        &'static str,
        #[source_code] Source,
        #[label("expected {0}")] SourceSpan,
    ),

    #[error("Missing node")]
    #[diagnostic(code("tm::missing_node"))]
    MissingValue(#[source_code] Source, #[label("missing value")] SourceSpan),

    #[error("Missing positional entry")]
    #[diagnostic(
        code("tm::missing_positional_entry"),
        help("child nodes are defined between '{{}}'")
    )]
    MissingChildNode(
        #[source_code] Source,
        #[label("expected child nodes")] SourceSpan,
    ),

    #[error("Invalid finder")]
    #[diagnostic(code("tm::invalid_finder"))]
    InvalidFinder(
        #[source_code] Source,
        #[label("Not a valid finder")] SourceSpan,
    ),

    #[error("Invalid height string")]
    #[diagnostic(code("tm::invalid_height_string"))]
    InvalidHeightString(
        #[source_code] Source,
        #[label(r#"expected string "full""#)] SourceSpan,
    ),

    #[error("Invalid height range")]
    #[diagnostic(code("tm::invalid_height_range"))]
    InvalidHeightRange(
        #[source_code] Source,
        #[label("expected number in range 1..=100 inclusively")] SourceSpan,
    ),

    #[error("Invalid percentage")]
    #[diagnostic(code("tm::invalid_percentage"))]
    InvalidPercentage(
        #[source_code] Source,
        #[label("expected a percentage from 1-100%")] SourceSpan,
    ),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Kdl(#[from] KdlError),

    #[error(transparent)]
    #[diagnostic()]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic()]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    #[diagnostic()]
    ParseIntError(#[from] std::num::ParseIntError),
}

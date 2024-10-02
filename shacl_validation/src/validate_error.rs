use oxiri::IriParseError;
use shacl_ast::{
    compiled::compiled_shacl_error::CompiledShaclError, shacl_parser_error::ShaclParserError,
};
use srdf::SRDFGraphError;
use thiserror::Error;

use crate::{
    constraints::constraint_error::ConstraintError,
    helper::helper_error::{SPARQLError, SRDFError},
};

#[derive(Error, Debug)]
pub enum ValidateError {
    #[error("Error during the SPARQL operation")]
    SRDF,
    #[error("TargetNode cannot be a Blank Node")]
    TargetNodeBlankNode,
    #[error("TargetClass should be an IRI")]
    TargetClassNotIri,
    #[error("Error when working with the SRDFGraph, {}", ._0)]
    Graph(#[from] SRDFGraphError),
    #[error("Error when parsing the SHACL Graph, {}", ._0)]
    ShaclParser(#[from] ShaclParserError),
    #[error("Error during the constraint evaluation")]
    Constraint(#[from] ConstraintError),
    #[error("Error parsing the IRI")]
    IriParse(#[from] IriParseError),
    #[error("Error during some I/O operation")]
    IO(#[from] std::io::Error),
    #[error("Error loading the Shapes")]
    Shapes(#[from] SRDFError),
    #[error("Error creating the SPARQL endpoint")]
    SPARQLCreation,
    #[error("The provided mode is not supported for the {} structure", ._0)]
    UnsupportedMode(String),
    #[error("Error during the SPARQL operation")]
    Sparql(#[from] SPARQLError),
    #[error("Implicit class not found")]
    ImplicitClassNotFound,
    #[error("Not yet implemented")]
    NotImplemented,
    #[error("Error during the compilation of the Schema, {}", ._0)]
    CompiledShacl(#[from] CompiledShaclError),
}

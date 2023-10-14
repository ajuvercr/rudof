use crate::ast::{serde_string_or_struct::*, SchemaJsonError};
use log::debug;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

use super::{Iri, SemAct, ShapeDecl, ShapeExpr};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Schema {
    #[serde(rename = "@context")]
    context: String,

    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub imports: Option<Vec<Iri>>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_opt_string_or_struct",
        deserialize_with = "deserialize_opt_string_or_struct"
    )]
    pub start: Option<ShapeExpr>,

    #[serde(default, rename = "startActs", skip_serializing_if = "Option::is_none")]
    pub start_acts: Option<Vec<SemAct>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shapes: Option<Vec<ShapeDecl>>,
}

impl Schema {
    pub fn parse_schema_buf(path_buf: &PathBuf) -> Result<Schema, SchemaJsonError> {
        let schema = {
            let schema_str = fs::read_to_string(&path_buf.as_path()).map_err(|e| {
                SchemaJsonError::ReadingPathError {
                    path_name: path_buf.display().to_string(),
                    error: e,
                }
            })?;
            serde_json::from_str::<Schema>(&schema_str).map_err(|e| SchemaJsonError::JsonError {
                path_name: path_buf.display().to_string(),
                error: e,
            })?
        };
        debug!("SchemaJson parsed: {:?}", schema);
        Ok(schema)
    }

    pub fn parse_schema_name(schema_name: &String, base: &Path) -> Result<Schema, SchemaJsonError> {
        let json_path = Path::new(&schema_name);
        let mut attempt = PathBuf::from(base);
        attempt.push(json_path);
        Self::parse_schema_buf(&attempt)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deser_user() {
        let str = r#"
        {
            "type": "Schema",
            "shapes": [
              {
                "type": "ShapeDecl",
                "id": "http://example.org/User",
                "shapeExpr": {
                  "type": "Shape",
                  "expression": {
                        "type": "TripleConstraint",
                        "predicate": "http://schema.org/name",
                        "valueExpr": {
                          "type": "NodeConstraint",
                          "datatype": "http://www.w3.org/2001/XMLSchema#string",
                          "length": 3
                        }
                  }
                }
              }
            ],
            "@context": "http://www.w3.org/ns/shex.jsonld"
          }
        "#;

        let schema: Schema = serde_json::from_str(&str).unwrap();
        let serialized = serde_json::to_string_pretty(&schema).unwrap();
        println!("{}", serialized);
        let schema_after_serialization = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schema, schema_after_serialization);
    }
}

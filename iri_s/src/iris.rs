use oxrdf::NamedNode;
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::str::FromStr;

use crate::IriSError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IriS(NamedNode);

impl IriS {
    pub fn new(str: &str) -> Result<IriS, IriSError> {
        let iri = NamedNode::new(str)?;
        Ok(IriS(iri))
    }

    pub fn new_unchecked(str: &str) -> IriS {
        let iri = NamedNode::new_unchecked(str);
        IriS(iri)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn from_named_node(iri: NamedNode) -> IriS {
        IriS(iri)
    }

    pub fn as_named_node(&self) -> &NamedNode {
        &self.0
    }

    pub fn extend(&self, str: &str) -> Result<Self, IriSError> {
        let s = format!("{}{}", self.0.as_str(), str);
        let iri = NamedNode::new(s)?;
        Ok(IriS(iri))
    }

    /*    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    } */
}

impl fmt::Display for IriS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for IriS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl FromStr for IriS {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_iri(s)
    }
}

fn parse_iri(s: &str) -> Result<IriS, IriSError> {
    IriS::new(s)
}

impl Default for IriS {
    fn default() -> Self {
        IriS::new_unchecked(&String::default())
    }
}

struct IriVisitor;

impl<'de> Visitor<'de> for IriVisitor {
    type Value = IriS;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an IRI")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        IriS::new(v).map_err(|e| E::custom(format!("Cannot parse as Iri: \"{v}\". Error: {e}")))
    }
}

impl<'de> Deserialize<'de> for IriS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IriVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_iris() {
        let iri = IriS::from_str("http://example.org/").unwrap();
        assert_eq!(iri.to_string(), "<http://example.org/>");
    }

    #[test]
    fn obtaining_iri_as_str() {
        let iri = IriS::from_str("http://example.org/p1").unwrap();
        assert_eq!(iri.as_str(), "http://example.org/p1");
    }
}

use srdf::iri::{IRI};
use srdf::bnode::{BNode};
use rio_api::model::{NamedNode,BlankNode};

pub struct IRIRio<'a> {
    iri: NamedNode<'a>
}
impl <'a> IRI<'a> for IRIRio<'a> {
    fn to_str(&self) -> &'a str { self.iri.iri }
}

pub struct BNodeRio<'a> {
    bnode: BlankNode<'a>
}
impl <'a> BNode<'a> for BNodeRio<'a> {
    fn label(&self) -> &'a str { self.bnode.id }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_iri() {
        let rdf_type = IRIRio { iri: NamedNode { iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" } };
        assert_eq!(rdf_type.to_str(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
    }
}

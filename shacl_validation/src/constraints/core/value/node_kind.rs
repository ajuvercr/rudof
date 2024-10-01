use indoc::formatdoc;
use shacl_ast::compiled::component::Nodekind;
use shacl_ast::node_kind::NodeKind;
use srdf::QuerySRDF;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::context::Context;
use crate::validation_report::result::ValidationResult;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for Nodekind {
    fn validate_native(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                let is_valid = match (
                    S::term_is_bnode(value_node),
                    S::term_is_iri(value_node),
                    S::term_is_literal(value_node),
                ) {
                    (true, false, false) => matches!(
                        self.node_kind(),
                        NodeKind::BlankNode
                            | NodeKind::BlankNodeOrIri
                            | NodeKind::BlankNodeOrLiteral
                    ),
                    (false, true, false) => matches!(
                        self.node_kind(),
                        NodeKind::Iri | NodeKind::IRIOrLiteral | NodeKind::BlankNodeOrIri
                    ),
                    (false, false, true) => matches!(
                        self.node_kind(),
                        NodeKind::Literal | NodeKind::IRIOrLiteral | NodeKind::BlankNodeOrLiteral
                    ),
                    _ => false,
                };

                if !is_valid {
                    let result =
                        ValidationResult::new(focus_node, &evaluation_context, Some(value_node));
                    Some(result)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Nodekind {
    fn validate_sparql(
        &self,
        evaluation_context: Context<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes.iter_value_nodes()
            .filter_map(move |(focus_node, value_node)| {
                let query = if S::term_is_iri(value_node) {
                    formatdoc! {"
                            PREFIX sh: <http://www.w3.org/ns/shacl#>
                            ASK {{ FILTER ({} IN ( sh:IRI, sh:BlankNodeOrIRI, sh:IRIOrLiteral ) ) }}
                        ", self.node_kind()
                    }
                } else if S::term_is_bnode(value_node) {
                    formatdoc! {"
                            PREFIX sh: <http://www.w3.org/ns/shacl#>
                            ASK {{ FILTER ({} IN ( sh:Literal, sh:BlankNodeOrLiteral, sh:IRIOrLiteral ) ) }}
                        ", self.node_kind()
                    }
                } else {
                    formatdoc! {"
                            PREFIX sh: <http://www.w3.org/ns/shacl#>
                            ASK {{ FILTER ({} IN ( sh:BlankNode, sh:BlankNodeOrIRI, sh:BlankNodeOrLiteral ) ) }}
                        ", self.node_kind()
                    }
                };

                let ask = match validation_context.store().query_ask(&query) {
                    Ok(ask) => ask,
                    Err(_) => return None,
                };

                if !ask {
                    Some(ValidationResult::new(focus_node, &evaluation_context, Some(value_node)))
                } else {
                    None
                }
            }).collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}

use prefixmap::IriRef;
use srdf::QuerySRDF;
use srdf::SRDF;
use std::sync::Arc;

use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::validation_report::result::LazyValidationIterator;
use crate::value_nodes::ValueNodes;

/// sh:lessThan specifies the condition that each value node is smaller than all
/// the objects of the triples that have the focus node as subject and the
/// value of sh:lessThan as predicate.
///
/// https://www.w3.org/TR/shacl/#LessThanConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct LessThan {
    iri_ref: IriRef,
}

impl LessThan {
    pub fn new(iri_ref: IriRef) -> Self {
        LessThan { iri_ref }
    }
}

impl< S: SRDF> DefaultConstraintComponent< S> for LessThan {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        unimplemented!()
    }
}

impl< S: QuerySRDF> SparqlConstraintComponent< S> for LessThan {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        unimplemented!()
    }
}

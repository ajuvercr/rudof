use shacl_ast::value::Value;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validation_report::result::LazyValidationIterator;
use crate::ValueNodes;

/// sh:hasValue specifies the condition that at least one value node is equal to
///  the given RDF term.
///
/// https://www.w3.org/TR/shacl/#HasValueConstraintComponent
#[allow(dead_code)] // TODO: Remove when it is used
pub(crate) struct HasValue {
    value: Value,
}

impl HasValue {
    pub fn new(value: Value) -> Self {
        HasValue { value }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for HasValue {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<S> {
        unimplemented!()
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for HasValue {
    fn evaluate_default(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<'_, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for HasValue {
    fn evaluate_sparql(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> LazyValidationIterator<'_, S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

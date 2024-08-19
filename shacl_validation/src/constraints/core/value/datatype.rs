use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::runner::ValidatorRunner;
use crate::validation_report::result::LazyValidationIterator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use iri_s::IriS;
use prefixmap::IriRef;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;
use std::sync::Arc;

/// sh:datatype specifies a condition to be satisfied with regards to the
/// datatype of each value node.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct Datatype<S: SRDFBasic> {
    datatype: S::IRI,
}

impl<S: SRDFBasic> Datatype<S> {
    pub fn new(iri_ref: IriRef) -> Self {
        Datatype {
            datatype: S::iri_s2iri(&IriS::new_unchecked(&iri_ref.to_string())),
        }
    }
}

impl< S: SRDFBasic, R: ValidatorRunner< S>> ConstraintComponent< S, R> for Datatype<S> {
    fn evaluate(
        & self,
        validation_context: Arc<ValidationContext< S, R>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        let results = value_nodes
            .iter_full()
            .flat_map(move |(focus_node, value_node)| {
                if let Some(literal) = S::term_as_literal(&value_node) {
                    if S::datatype(&literal) != self.datatype {
                        let result = ValidationResult::new(
                            &focus_node,
                            Arc::clone(&evaluation_context),
                            Some(&value_node),
                        );
                        Some(result)
                    } else {
                        None
                    }
                } else {
                    let result = ValidationResult::new(
                        &focus_node,
                        Arc::clone(&evaluation_context),
                        Some(&value_node),
                    );
                    Some(result)
                }
            });

        LazyValidationIterator::new(results)
    }
}

impl< S: SRDF> DefaultConstraintComponent< S> for Datatype<S> {
    fn evaluate_default(
        & self,
        validation_context: Arc<ValidationContext< S, DefaultValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

impl< S: QuerySRDF> SparqlConstraintComponent< S> for Datatype<S> {
    fn evaluate_sparql(
        & self,
        validation_context: Arc<ValidationContext< S, QueryValidatorRunner>>,
        evaluation_context: Arc<EvaluationContext<>>,
        value_nodes: Arc<ValueNodes< S>>,
    ) -> LazyValidationIterator< S> {
        self.evaluate(validation_context, evaluation_context, value_nodes)
    }
}

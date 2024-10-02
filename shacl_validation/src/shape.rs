use shacl_ast::compiled::node_shape::CompiledNodeShape;
use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::SRDFBasic;

use crate::runner::ValidatorRunner;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResults;
use crate::Targets;
use crate::ValueNodes;

pub struct ShapeValidation<'a, S: SRDFBasic> {
    store: &'a S,
    runner: &'a dyn ValidatorRunner<S>,
    shape: &'a CompiledShape<S>,
    focus_nodes: Targets<S>,
}

impl<'a, S: SRDFBasic> ShapeValidation<'a, S> {
    pub fn new(
        store: &'a S,
        runner: &'a dyn ValidatorRunner<S>,
        shape: &'a CompiledShape<S>,
        targets: Option<&'a Targets<S>>,
    ) -> Self {
        let focus_nodes = match targets {
            Some(targets) => targets.to_owned(),
            None => shape.focus_nodes(store, runner),
        };

        ShapeValidation {
            store,
            runner,
            shape,
            focus_nodes,
        }
    }

    pub fn validate(&self) -> Result<ValidationResults<S>, ValidateError> {
        if *self.shape.is_deactivated() {
            // skipping because it is deactivated
            return Ok(ValidationResults::default());
        }

        let components = self.validate_components()?;
        let property_shapes = self.validate_property_shapes()?;
        let validation_results = components.into_iter().chain(property_shapes);

        Ok(ValidationResults::new(validation_results))
    }

    fn validate_components(&self) -> Result<ValidationResults<S>, ValidateError> {
        // 1. First we compute the ValueNodes; that is, the set of nodes that
        //    are going to be used during the validation stages. This set of
        //    nodes is obtained from the set of focus nodes
        let value_nodes = self
            .shape
            .value_nodes(self.store, &self.focus_nodes, self.runner);

        let results = self.shape.components().iter().flat_map(move |component| {
            self.runner
                .evaluate(self.store, component, &value_nodes)
                .unwrap_or_else(|_| ValidationResults::default())
        });

        Ok(ValidationResults::new(results))
    }

    fn validate_property_shapes(&self) -> Result<ValidationResults<S>, ValidateError> {
        let evaluated_shapes = self.shape.property_shapes().iter().flat_map(|shape| {
            ShapeValidation::new(self.store, self.runner, shape, Some(&self.focus_nodes))
                .validate()
                .ok()
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
        });

        Ok(ValidationResults::new(evaluated_shapes))
    }
}

pub trait FocusNodesOps<S: SRDFBasic> {
    fn focus_nodes(&self, store: &S, runner: &dyn ValidatorRunner<S>) -> Targets<S>;
}

impl<S: SRDFBasic> FocusNodesOps<S> for CompiledShape<S> {
    fn focus_nodes(&self, store: &S, runner: &dyn ValidatorRunner<S>) -> Targets<S> {
        runner
            .focus_nodes(store, self, self.targets())
            .expect("Failed to retrieve focus nodes")
    }
}

pub trait ValueNodesOps<S: SRDFBasic> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &Targets<S>,
        runner: &dyn ValidatorRunner<S>,
    ) -> ValueNodes<S>;
}

impl<S: SRDFBasic> ValueNodesOps<S> for CompiledShape<S> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &Targets<S>,
        runner: &dyn ValidatorRunner<S>,
    ) -> ValueNodes<S> {
        match self {
            CompiledShape::NodeShape(ns) => ns.value_nodes(store, focus_nodes, runner),
            CompiledShape::PropertyShape(ps) => ps.value_nodes(store, focus_nodes, runner),
        }
    }
}

impl<S: SRDFBasic> ValueNodesOps<S> for CompiledNodeShape<S> {
    fn value_nodes(
        &self,
        _: &S,
        focus_nodes: &Targets<S>,
        _: &dyn ValidatorRunner<S>,
    ) -> ValueNodes<S> {
        let value_nodes = focus_nodes.iter().map(|focus_node| {
            (
                focus_node.clone(),
                Targets::new(std::iter::once(focus_node.clone())),
            )
        });

        ValueNodes::new(value_nodes)
    }
}

impl<S: SRDFBasic> ValueNodesOps<S> for CompiledPropertyShape<S> {
    fn value_nodes(
        &self,
        store: &S,
        focus_nodes: &Targets<S>,
        runner: &dyn ValidatorRunner<S>,
    ) -> ValueNodes<S> {
        let value_nodes = focus_nodes.iter().filter_map(move |focus_node| {
            runner
                .path(store, self, focus_node)
                .ok()
                .map(|targets| (focus_node.clone(), targets))
        });

        ValueNodes::new(value_nodes)
    }
}

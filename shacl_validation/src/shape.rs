use shacl_ast::node_shape::NodeShape;
use shacl_ast::property_shape::PropertyShape;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::runner::ValidatorRunner;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub trait Validate<S: SRDF + SRDFBasic> {
    fn validate(
        &self,
        runner: &impl ValidatorRunner<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ValidateError>;
}

impl<S: SRDF + SRDFBasic> Validate<S> for NodeShape {
    fn validate(
        &self,
        runner: &impl ValidatorRunner<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ValidateError> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(());
        }

        for component in self.components() {
            let value_nodes = runner.focus_nodes(self.targets())?;
            runner.evaluate(component, value_nodes, report)?;
        }

        Ok(())
    }
}

impl<S: SRDF + SRDFBasic> Validate<S> for PropertyShape {
    fn validate(
        &self,
        runner: &impl ValidatorRunner<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ValidateError> {
        if *self.is_deactivated() {
            // skipping because it is deactivated
            return Ok(());
        }

        for component in self.components() {
            let focus_nodes = runner.focus_nodes(self.targets())?;
            let value_nodes = runner.path(self.path())?;
            runner.evaluate(component, value_nodes, report)?;
        }

        Ok(())
    }
}

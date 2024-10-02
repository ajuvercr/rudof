use constraint_error::ConstraintError;
use shacl_ast::compiled::component::Component;
use srdf::QuerySRDF;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::runner::ValidatorRunner;
use crate::validation_report::result::ValidationResults;
use crate::ValueNodes;

pub mod constraint_error;
pub mod core;

pub(crate) trait Validator<S: SRDFBasic> {
    fn validate(
        &self,
        store: &S,
        runner: impl ValidatorRunner<S>,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError>;
}

pub trait NativeValidator<S: SRDF> {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError>;
}

pub trait SparqlValidator<S: QuerySRDF> {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError>;
}

macro_rules! generate_deref_fn {
    ($enum_name:ident, $($variant:ident),+) => {
        fn deref(&self) -> &Self::Target {
            match self {
                $( $enum_name::$variant(inner) => inner, )+
            }
        }
    };
}

pub trait NativeDeref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

impl<S: SRDF + 'static> NativeDeref for Component<S> {
    type Target = dyn NativeValidator<S>;

    generate_deref_fn!(
        Component,
        Class,
        Datatype,
        NodeKind,
        MinCount,
        MaxCount,
        MinExclusive,
        MaxExclusive,
        MinInclusive,
        MaxInclusive,
        MinLength,
        MaxLength,
        Pattern,
        UniqueLang,
        LanguageIn,
        Equals,
        Disjoint,
        LessThan,
        LessThanOrEquals,
        Or,
        And,
        Not,
        Xone,
        Closed,
        Node,
        HasValue,
        In,
        QualifiedValueShape
    );
}

pub trait SparqlDeref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

impl<S: QuerySRDF + 'static> SparqlDeref for Component<S> {
    type Target = dyn SparqlValidator<S>;

    generate_deref_fn!(
        Component,
        Class,
        Datatype,
        NodeKind,
        MinCount,
        MaxCount,
        MinExclusive,
        MaxExclusive,
        MinInclusive,
        MaxInclusive,
        MinLength,
        MaxLength,
        Pattern,
        UniqueLang,
        LanguageIn,
        Equals,
        Disjoint,
        LessThan,
        LessThanOrEquals,
        Or,
        And,
        Not,
        Xone,
        Closed,
        Node,
        HasValue,
        In,
        QualifiedValueShape
    );
}

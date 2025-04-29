use std::fmt::Display;

use shex_ast::{
    ir::{node_constraint::NodeConstraint, shape::Shape, shape_expr::ShapeExpr},
    Node,
};

/// Reason represents justifications about why a node conforms to some shape
#[derive(Debug, Clone)]
pub enum Reason {
    NodeConstraintPassed { node: Node, nc: NodeConstraint },
    ShapeAndPassed { node: Node, se: ShapeExpr },
    ShapePassed { node: Node, shape: Shape },
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::NodeConstraintPassed { node, nc } => {
                write!(f, "Node constraint passed. Node: {node}, Constraint: {nc}",)
            }
            Reason::ShapeAndPassed { node, se } => {
                write!(f, "AND passed. Node {node}, and: {se}")
            }
            Reason::ShapePassed { node, shape } => {
                write!(f, "Shape passed. Node {node}, shape: {shape}")
            }
        }
    }
}

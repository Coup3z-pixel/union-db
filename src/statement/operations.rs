use std::any::Any;

use super::{node::Node, types::StatementExecutionError};

pub mod manipulation;
pub mod definition;

pub trait Operation {
    fn calculate(&self, left_node: &Node, right_node: &Node) -> Result<Node, StatementExecutionError>;
    fn as_any(&self) -> &dyn Any;
    fn get_name(&self) -> &str;
}

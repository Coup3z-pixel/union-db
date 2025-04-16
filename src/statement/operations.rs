use std::any::Any;

use super::{node::Node, types::StatementExecutionError};

pub mod manipulation;
pub mod definition;

pub trait Operation {
    fn execute(&self, left_node: &Node, right_node: &Node) -> Result<Node, StatementExecutionError>;
    fn as_any(&self) -> &dyn Any;
    fn get_name(&self) -> &str;
    fn format_operation_str(&self, left_name: &str, right_name: &str) -> String;
}

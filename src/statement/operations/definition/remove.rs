use crate::statement::operations::Operation;
use crate::statement::operations::Any;
use crate::statement::node::Node;

use crate::statement::types::StatementExecutionError;

pub struct Remove ();

impl Operation for Remove {
    fn execute(&self, _left_node: &Node, right_node: &Node) -> Result<Node, StatementExecutionError> {
        match right_node {
            Node::Set(_) => Ok(Node::Set("removed".to_string())),
            _ => Err(StatementExecutionError),
        }
    }

    fn format_operation_str(&self, _left_name: &str, right_name: &str) -> String {
        return format!("define_{}", right_name);
    }

    fn as_any(&self) -> &dyn Any {
        &Self
    }

    fn get_name(&self) -> &str {
        "define"
    }

}

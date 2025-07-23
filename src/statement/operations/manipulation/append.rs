use crate::statement::operations::Operation;
use crate::statement::operations::Any;
use crate::statement::node::Node;

use crate::statement::types::StatementExecutionError;

pub struct Append ();

impl Operation for Append {
    fn execute(&self, left_node: &Node, right_node: &Node) -> Result<Node, StatementExecutionError> {
        let set_name = match left_node {
            Node::Set(name) => name,
            _ => return Err(StatementExecutionError)
        };

        let value = match right_node {
            Node::Set(element) => element,
            _ => return Err(StatementExecutionError)
        };

        todo!();
    }
    
    fn format_operation_str(&self, _: &str, right_name: &str) -> String {
        return format!("define_{}", right_name);
    }

    fn as_any(&self) -> &dyn Any {
        &Self
    }

    fn get_name(&self) -> &str {
        "define"
    }

}

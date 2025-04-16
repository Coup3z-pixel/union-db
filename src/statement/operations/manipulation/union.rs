use crate::statement::node::Node;
use crate::statement::operations::Operation;
use crate::statement::operations::Any;
use crate::statement::types::StatementExecutionError;

pub struct Union ();

impl Operation for Union {
    fn calculate(&self, _left_node: &Node, _right_node: &Node) -> Result<Node, StatementExecutionError> {
        Ok(Node::Set("asdf".to_string()))
    }

    fn as_any(&self) -> &dyn Any {
        &Self
    }

    fn get_name(&self) -> &str {
        "union"
    }
}

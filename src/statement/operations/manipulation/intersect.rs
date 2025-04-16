use crate::statement::node::Node;
use crate::statement::operations::Operation;
use crate::statement::operations::Any;
use crate::statement::types::StatementExecutionError;

pub struct Intersect ();

impl Operation for Intersect  {
    fn calculate(&self, left_node: &Node, right_node: &Node) -> Result<Node, StatementExecutionError> {
       Ok(Node::Set("asdfasfd".to_string()))
    }

    fn as_any(&self) -> &dyn Any {
        &Self
    }

    fn get_name(&self) -> &str {
        "intersect"
    }
}

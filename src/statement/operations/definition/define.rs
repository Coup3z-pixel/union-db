use crate::statement::operations::Operation;
use crate::statement::operations::Any;
use crate::statement::node::Node;

use crate::statement::types::StatementExecutionError;
use crate::storage::model::element::Element;
use crate::storage::model::set::Set;

pub struct Define ();

impl Operation for Define {
    fn execute(&self, _left_node: &Node, right_node: &Node) -> Result<Node, StatementExecutionError> {

        let new_set = match right_node {
            Node::Set(name) => Set { set_name: name.to_string(), element: Element {} },
            _ => return Err(StatementExecutionError),
        };

        return Ok(Node::Set(new_set.set_name));
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

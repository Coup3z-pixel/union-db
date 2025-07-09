use crate::statement::operations::Operation;
use crate::statement::operations::Any;
use crate::statement::node::Node;

use crate::statement::types::StatementExecutionError;
use crate::storage::model::set::Set;

pub struct Define ();

impl Operation for Define {
    fn execute(&self, _left_node: &Node, right_node: &Node) -> Result<Node, StatementExecutionError> {
        let new_set = match right_node {
            Node::Set(name) => Set::new(&name),
            _ => return Err(StatementExecutionError),
        };

        println!("Created New Set");

        return Ok(Node::Set(new_set.get_set_name().to_string()));
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

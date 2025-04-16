use crate::statement::node::Node;
use crate::statement::operations::Operation;
use crate::statement::operations::Any;
use crate::statement::types::StatementExecutionError;

pub struct Union ();

impl Operation for Union {
    fn execute(&self, left_node: &Node, right_node: &Node) -> Result<Node, StatementExecutionError> {
        let left_set_name = match left_node {
            Node::Set(name) => name,
            _ => return Err(StatementExecutionError)
        };

        let right_set_name = match right_node {
            Node::Set(name) => name,
            _ => return Err(StatementExecutionError)
        };

        let str_identifier = self.format_operation_str(left_set_name, right_set_name);

        println!("{}", str_identifier);

        Ok(Node::Set(str_identifier))
    }

    fn as_any(&self) -> &dyn Any {
        &Self
    }

    fn get_name(&self) -> &str {
        "union"
    }

    fn format_operation_str(&self, left_name: &str, right_name: &str) -> String {
        return format!("{}_union_{}", left_name, right_name);
    }
}

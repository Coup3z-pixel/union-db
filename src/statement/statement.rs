use super::node::Node;
use super::types::StatementExecutionError;

pub struct Statement {
    expression_tree: Option<Box<Node>>
}

impl Statement {
    pub fn new(head: Option<Box<Node>>) -> Self {
        Self { expression_tree: head }
    }

    pub fn print_tree(&self) {
        self.print_tree_helper(self.expression_tree.as_deref());
    }

    fn print_tree_helper(&self, curr_node: Option<&Node>) -> () {
        match curr_node {
            Some(node) => match node {
                Node::Set(set_name) => {
                    println!("{}", set_name);
                }
                Node::Operation(operation_type, left_node, right_node) => {
                    self.print_tree_helper(left_node.as_deref());
                    println!("{}", operation_type.get_name());
                    self.print_tree_helper(right_node.as_deref());
                }
            },
            _ => {
                println!("");
            }
        }
    }

    pub fn execute(&self) -> Result<Node, StatementExecutionError> {
        return self.execute_helper(self.expression_tree.as_deref());
    } 

    fn execute_helper(&self, curr_node: Option<&Node>) -> Result<Node, StatementExecutionError> {
        match curr_node {
            Some(node) => match node {
                Node::Operation(operation, Some(left_node), Some(right_node)) => {

                    let left_node_result = match left_node.as_ref() {
                        Node::Operation(_, Some(_), Some(_)) => match self.execute_helper(Some(left_node)) {
                            Ok(node) => node,
                            Err(StatementExecutionError) => return Err(StatementExecutionError)
                        },
                        Node::Set(name) => Node::Set(name.to_string()), 
                        _ => return Err(StatementExecutionError),
                    };

                    let right_node_result = match right_node.as_ref() {
                        Node::Operation(_, Some(_), Some(_)) => match self.execute_helper(Some(right_node)) {
                            Ok(node) => node,
                            Err(StatementExecutionError) => return Err(StatementExecutionError)
                        },
                        Node::Set(name) => Node::Set(name.to_string()), 
                        _ => return Err(StatementExecutionError),
                    };

                    return operation.as_ref().calculate(&left_node_result, &right_node_result);
                },
                Node::Set(name) => Ok(Node::Set(name.to_string())),
                _ => Err(StatementExecutionError)
            },
            _ => Err(StatementExecutionError),
        }
    }
}

use super::node::Node;
use super::types;

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
                    println!("{}", types::convert_type_to_symbol(operation_type));
                    self.print_tree_helper(right_node.as_deref());
                }
            },
            None => {
                println!("");
            }
        }
    }
}

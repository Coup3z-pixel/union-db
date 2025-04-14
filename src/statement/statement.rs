use super::node::Node;

pub struct Statement {
    expression_tree: Option<Box<Node>>
}

impl Statement {
    pub fn new(head: Option<Box<Node>>) -> Self {
        Self { expression_tree: head }
    }
}

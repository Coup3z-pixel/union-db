use super::operations::OperationType;

pub enum Node {
    Set(String),
    Operation(OperationType, Option<Box<Node>>, Option<Box<Node>>)
}

use crate::storage::model::element::Element;

use super::operations::Operation;

pub enum Node {
    Set(String),
    Operation(Box<dyn Operation>, Option<Box<Node>>, Option<Box<Node>>),
}

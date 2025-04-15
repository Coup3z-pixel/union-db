pub mod manipulation;
pub mod definition;

pub enum OperationType {
    Minus,
    Intersect,
    Union,
    Placeholder
}

trait Operation {
    fn operation(&self);
}

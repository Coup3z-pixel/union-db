pub mod intersect;
pub mod minus;
pub mod union;

trait Operation {
    fn operation(&self);
}

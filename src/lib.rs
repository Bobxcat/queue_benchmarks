pub mod std_lib;

pub trait Queue<T> {
    fn init_empty() -> Self;
    /// Pushes to the back of the queue
    fn push(&mut self, val: T);
    /// Pops from the front of the queue
    fn pop(&mut self) -> Option<T>;
}

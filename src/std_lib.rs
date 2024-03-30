use std::collections::{LinkedList, VecDeque};

use crate::Queue;

impl<T> Queue<T> for VecDeque<T> {
    fn init_empty() -> Self {
        Self::new()
    }

    fn push(&mut self, val: T) {
        self.push_back(val)
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }
}

impl<T> Queue<T> for Vec<T> {
    fn init_empty() -> Self {
        Self::new()
    }

    fn push(&mut self, val: T) {
        self.push(val)
    }

    fn pop(&mut self) -> Option<T> {
        (self.len() > 0).then(|| self.remove(0))
    }
}

impl<T> Queue<T> for LinkedList<T> {
    fn init_empty() -> Self {
        Self::new()
    }

    fn push(&mut self, val: T) {
        self.push_back(val)
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }
}

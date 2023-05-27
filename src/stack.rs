
#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn length(&self) -> usize {
        self.stack.len()
    }
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
    pub fn last(&self) ->Option<&T> {
        self.stack.first()
    }
    pub fn remove_first(&mut self) {
        self.stack.remove(0);
    }
}

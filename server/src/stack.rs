pub struct MyStack<T> {

    vec: Vec<T>,
}

pub trait Stack<T> {

    fn new() -> Self;

    fn size(&self) -> usize;

    fn empty(&self) -> bool;

    fn push(&mut self, e: T);

    fn pop(&mut self) -> Option<T>;

    fn top(&self) -> Option<&T>;
}

impl<T> Stack<T> for MyStack<T> {
    fn new() -> Self {
        MyStack { vec: Vec::new() }
    }

    fn size(&self) -> usize {
        self.vec.len()
    }

    fn empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn push(&mut self, e: T) {
        self.vec.push(e)
    }

    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    fn top(&self) -> Option<&T> {
        self.vec.last()
    }
}

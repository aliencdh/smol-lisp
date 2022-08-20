use std::collections::VecDeque;

#[derive(Clone)]
pub struct Zipper<T: Clone> {
    left: VecDeque<T>,
    right: VecDeque<T>,
}
impl<T: Clone> Zipper<T> {
    /// Moves the zipper to the left, if possible, popping an item from the left
    /// buffer and prepending it to the right buffer.
    /// If the left buffer is empty, returns false.
    pub fn move_left(&mut self) -> bool {
        if let Some(item) = self.left.pop_back() {
            self.right.push_front(item);
            return true;
        }
        false
    }

    /// Moves the zipper to the right, if possible, popping an item from the right
    /// buffer and prepending it to the left buffer.
    /// If the right buffer is empty, returns false.
    pub fn move_right(&mut self) -> bool {
        if let Some(item) = self.right.pop_front() {
            self.left.push_back(item);
            return true;
        }
        false
    }

    /// Returns a shared reference to the item to the left, if it exists.
    pub fn left(&self) -> Option<&T> {
        self.left.back()
    }

    /// Returns a shared reference to the item to the right, if it exists.
    pub fn right(&self) -> Option<&T> {
        self.right.front()
    }

    /// Returns an exclusive reference to the item to the left, if it exists.
    pub fn left_mut(&mut self) -> Option<&mut T> {
        self.left.back_mut()
    }

    /// Returns an exclusive reference to the item to the right, if it exists.
    pub fn right_mut(&mut self) -> Option<&mut T> {
        self.right.front_mut()
    }

    /// Inserts an item to the left.
    pub fn ins_left(&mut self, item: T) {
        self.left.push_back(item);
    }

    /// Inserts an item to the right.
    pub fn ins_right(&mut self, item: T) {
        self.right.push_front(item);
    }
}
impl<T: Clone> From<&[T]> for Zipper<T> {
    fn from(src: &[T]) -> Self {
        Self {
            left: VecDeque::new(),
            right: VecDeque::from(src.to_vec()),
        }
    }
}

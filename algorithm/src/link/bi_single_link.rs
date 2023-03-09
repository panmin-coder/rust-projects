pub struct StackList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> StackList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head.take();
        self.head = Some(node);
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node { elem, next: None });
        self.push_node(node);
    }

    pub fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node
        })
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node| node.elem)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Default for StackList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for StackList<T> {
    fn drop(&mut self) {
        while self.pop_node().is_some() {}
    }
}

pub struct List<T> {
    left: StackList<T>,
    right: StackList<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            left: StackList::new(),
            right: StackList::new(),
        }
    }

    pub fn push_left(&mut self, elem: T) {
        self.left.push(elem);
    }

    pub fn push_right(&mut self, elem: T) {
        self.right.push(elem);
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.left.pop()
    }

    pub fn pop_right(&mut self) -> Option<T> {
        self.right.pop()
    }

    pub fn peek_left(&self) -> Option<&T> {
        self.left.peek()
    }

    pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        self.left.peek_mut()
    }

    pub fn peek_right(&self) -> Option<&T> {
        self.right.peek()
    }

    pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.right.peek_mut()
    }

    pub fn go_left(&mut self) -> bool {
        self.left
            .pop_node()
            .map(|node| {
                self.right.push_node(node);
            })
            .is_some()
    }

    pub fn go_right(&mut self) -> bool {
        self.right
            .pop_node()
            .map(|node| {
                self.left.push_node(node);
            })
            .is_some()
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.left.pop_node().is_some() {}
        while self.right.pop_node().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn walk_aboot() {
        let mut list = List::new(); // [_]

        list.push_left(0); // [0,_]
        list.push_right(1); // [0, _, 1]
        assert_eq!(list.peek_left(), Some(&0));
        assert_eq!(list.peek_right(), Some(&1));

        list.push_left(2); // [0, 2, _, 1]
        list.push_left(3); // [0, 2, 3, _, 1]
        list.push_right(4); // [0, 2, 3, _, 4, 1]

        while list.go_left() {} // [_, 0, 2, 3, 4, 1]

        assert_eq!(list.pop_left(), None);
        assert_eq!(list.pop_right(), Some(0)); // [_, 2, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(2)); // [_, 3, 4, 1]

        list.push_left(5); // [5, _, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(3)); // [5, _, 4, 1]
        assert_eq!(list.pop_left(), Some(5)); // [_, 4, 1]
        assert_eq!(list.pop_right(), Some(4)); // [_, 1]
        assert_eq!(list.pop_right(), Some(1)); // [_]

        assert_eq!(list.pop_right(), None);
        assert_eq!(list.pop_left(), None);
    }
}
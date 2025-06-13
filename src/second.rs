type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod my_test {
    use crate::second;

    use super::List;

    #[test]
    fn basics() {
        let mut my_list = List::new();

        assert_eq!(my_list.pop(), None);

        my_list.push(1);
        my_list.push(2);
        my_list.push(3);

        assert_eq!(my_list.pop(), Some(3));
        assert_eq!(my_list.pop(), Some(2));

        my_list.push(4);
        my_list.push(5);

        assert_eq!(my_list.pop(), Some(5));
        assert_eq!(my_list.pop(), Some(4));
    }

    #[test]
    fn peek() {
        let mut my_list: second::List<i32> = List::new();
    }
}

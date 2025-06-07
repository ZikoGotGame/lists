use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod my_test {
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
}

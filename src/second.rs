use std::mem;

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        // Todo
    }
}

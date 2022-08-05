use std::collections::VecDeque;

struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: u32) -> Self {
        Node { value, left: Option::None, right: Option::None }
    }

    fn left(self: Self, l: Node) -> Self {
        Node { value: self.value, left: Some(Box::new(l)), right: self.right }
    }

    fn right(self: Self, r: Node) -> Self {
        Node { value: self.value, left: self.left, right: Some(Box::new(r)) }
    }
}

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();
    let mut queue: VecDeque<&Node> = VecDeque::new();

    queue.push_back(root);
    while !queue.is_empty() {
        let n = queue.pop_front().unwrap();

        out.push(n.value);

        if let Some(ref left) = n.left { queue.push_back(&*left) }

        if let Some(ref right) = n.right { queue.push_back(&*right) }
    }

    out
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn root_only() {
        assert_eq!(tree_by_levels(&Node::new(42)),
                   [42],
                   "\nYour result (left) didn't match the expected output (right).");
    }

    #[test]
    fn complete_tree() {
        let root = Node::new(1)
            .left(Node::new(2)
                .left(Node::new(4))
                .right(Node::new(5)))
            .right(Node::new(3)
                .left(Node::new(6)));
        assert_eq!(tree_by_levels(&root),
                   [1, 2, 3, 4, 5, 6],
                   "\nYour result (left) didn't match the expected output (right).");
    }
}
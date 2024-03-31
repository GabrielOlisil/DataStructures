struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
    fn inorder_traversal(&self) {
        if let Some(left) = &self.left {
            left.inorder_traversal();
        }
        println!("{}", self.data);
        if let Some(right) = &self.right {
            right.inorder_traversal();
        }
    }
}

fn insert(root: &mut Option<Box<Node>>, data: i32) {
    let temp = Box::new(Node::new(data));
    match root {
        Some(node) => {
            if data > node.data {
                insert(&mut node.right, data);
            } else {
                insert(&mut node.left, data);
            }
        }
        None => {
            *root = Some(temp);
        }
    }
}

fn main() {
    let numeros = vec![2, 4, 1, 7, 5, 9, 3];

    let mut arvore: Option<Box<Node>> = None;

    for data in numeros {
        insert(&mut arvore, data);
    }

    arvore.unwrap().inorder_traversal();
}

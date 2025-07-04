#[derive(Debug)]
struct Node {
    key: i32,
    val: i32,
    prio: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32, val: i32, prio: u32) -> Box<Self> {
        Box::new(Node {
            key,
            val,
            prio,
            left: None,
            right: None,
        })
    }

    fn insert(node: Option<Box<Node>>, key: i32, val: i32, rng: &mut XorShift32) -> Option<Box<Node>> {
        match node {
            None => Some(Node::new(key, val, rng.next())),
            Some(mut n) => {
                if key < n.key {
                    n.left = Node::insert(n.left.take(), key, val, rng);
                    if n.left.as_ref().unwrap().prio > n.prio {
                        return Some(Self::rotate_right(n));
                    }
                } else {
                    n.right = Node::insert(n.right.take(), key, val, rng);
                    if n.right.as_ref().unwrap().prio > n.prio {
                        return Some(Self::rotate_left(n));
                    }
                }
                Some(n)
            }
        }
    }

    fn erase_all(node: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
        match node {
            None => None,
            Some(mut n) => {
                n.left = Self::erase_all(n.left.take(), key);
                n.right = Self::erase_all(n.right.take(), key);

                if n.key == key {
                    return Self::merge(n.left.take(), n.right.take());
                }
                Some(n)
            }
        }
    }

    fn merge(a: Option<Box<Node>>, b: Option<Box<Node>>) -> Option<Box<Node>> {
        match (a, b) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut lnode), Some(mut rnode)) => {
                if lnode.prio > rnode.prio {
                    lnode.right = Self::merge(lnode.right.take(), Some(rnode));
                    Some(lnode)
                } else {
                    rnode.left = Self::merge(Some(lnode), rnode.left.take());
                    Some(rnode)
                }
            }
        }
    }

    fn rotate_left(mut node: Box<Node>) -> Box<Node> {
        let mut new_root = node.right.take().unwrap();
        node.right = new_root.left.take();
        new_root.left = Some(node);
        new_root
    }

    fn rotate_right(mut node: Box<Node>) -> Box<Node> {
        let mut new_root = node.left.take().unwrap();
        node.left = new_root.right.take();
        new_root.right = Some(node);
        new_root
    }

    fn inorder(node: &Option<Box<Node>>) {
        if let Some(n) = node {
            Self::inorder(&n.left);
            print!("({}, {}) ", n.key, n.val);
            Self::inorder(&n.right);
        }
    }
}

pub struct Treap {
    root: Option<Box<Node>>,
    rng: XorShift32,
}

impl Treap {
    pub fn new() -> Self {
        Treap {
            root: None,
            rng: XorShift32::new(0x12345678),
        }
    }

    pub fn insert(&mut self, key: i32, val: i32) {
        self.root = Node::insert(self.root.take(), key, val, &mut self.rng);
    }

    /// 指定された key をすべて削除する（存在しなければ無視）
    pub fn erase(&mut self, key: i32) {
        self.root = Node::erase_all(self.root.take(), key);
    }

    /// 指定 key をすべて削除し、(key, value) を1つだけ挿入する
    pub fn update(&mut self, key: i32, val: i32) {
        self.erase(key);
        self.insert(key, val);
    }

    pub fn inorder(&self) {
        Node::inorder(&self.root);
        println!();
    }
}

pub struct XorShift32 {
    state: u32,
}

impl XorShift32 {
    pub fn new(seed: u32) -> Self {
        XorShift32 { state: seed }
    }

    pub fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
}

fn main() {
    let mut treap = Treap::new();

    treap.insert(5, 100);
    treap.insert(3, 200);
    treap.insert(3, 201);
    treap.insert(3, 202);
    treap.insert(7, 300);

    println!("Before erase:");
    treap.inorder(); // => (3,200) (3,201) (3,202) (5,100) (7,300)

    treap.erase(3);

    println!("After erase(3):");
    treap.inorder(); // => (5,100) (7,300)

    treap.update(7, 777);

    println!("After update(7, 777):");
    treap.inorder(); // => (5,100) (7,777)
}

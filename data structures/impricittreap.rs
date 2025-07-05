fn main() {
    let mut treap = ImplicitTreap::new(|&a, &b| a + b, 0);

    treap.insert(10, 1);
    treap.insert(20, 2);
    treap.insert(15, 3);
    treap.insert(25, 4);

    println!("All sum: {}", treap.all_prod()); // 10

    println!("prod(10, 20): {}", treap.prod(10, 20)); // 1 + 3 = 4
    println!("prod(15, 30): {}", treap.prod(15, 30)); // 3 + 2 + 4 = 9

    treap.update(15, 10); // 3 -> 10
    println!("After update: prod(10,30): {}", treap.prod(10, 30)); // 1 + 10 + 2 + 4 = 17

    // Split and Merge demo
    let (left, right) = treap.split(20);
    println!("Left split sum: {}", left.all_prod());  // 1 + 10 = 11
    println!("Right split sum: {}", right.all_prod()); // 2 + 4 = 6

    let merged = left.merge(right);
    println!("Merged sum: {}", merged.all_prod()); // 17
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

#[derive(Debug)]
struct Node<T: Clone + std::fmt::Debug> {
    key: i32,
    val: T,
    sum: T,
    prio: u32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Clone + std::fmt::Debug> Node<T> {
    fn new(key: i32, val: T, prio: u32, _op: fn(&T, &T) -> T) -> Box<Self> {
        let sum = val.clone();
        Box::new(Node {
            key,
            val,
            sum,
            prio,
            left: None,
            right: None,
        })
    }

    fn update(&mut self, op: fn(&T, &T) -> T, _e: &T) {
        self.sum = self.val.clone();
        if let Some(l) = &self.left {
            self.sum = op(&l.sum, &self.sum);
        }
        if let Some(r) = &self.right {
            self.sum = op(&self.sum, &r.sum);
        }
    }

    fn rotate_left(mut node: Box<Node<T>>, op: fn(&T, &T) -> T, e: &T) -> Box<Node<T>> {
        let mut new_root = node.right.take().unwrap();
        node.right = new_root.left.take();
        node.update(op, e);
        new_root.left = Some(node);
        new_root.update(op, e);
        new_root
    }

    fn rotate_right(mut node: Box<Node<T>>, op: fn(&T, &T) -> T, e: &T) -> Box<Node<T>> {
        let mut new_root = node.left.take().unwrap();
        node.left = new_root.right.take();
        node.update(op, e);
        new_root.right = Some(node);
        new_root.update(op, e);
        new_root
    }

    fn insert(
        node: Option<Box<Node<T>>>,
        key: i32,
        val: T,
        rng: &mut XorShift32,
        op: fn(&T, &T) -> T,
        e: &T,
    ) -> Option<Box<Node<T>>> {
        match node {
            None => Some(Node::new(key, val, rng.next(), op)),
            Some(mut n) => {
                if key < n.key {
                    n.left = Self::insert(n.left.take(), key, val, rng, op, e);
                    if n.left.as_ref().unwrap().prio > n.prio {
                        return Some(Self::rotate_right(n, op, e));
                    }
                } else {
                    n.right = Self::insert(n.right.take(), key, val, rng, op, e);
                    if n.right.as_ref().unwrap().prio > n.prio {
                        return Some(Self::rotate_left(n, op, e));
                    }
                }
                n.update(op, e);
                Some(n)
            }
        }
    }

    fn erase_all(
        node: Option<Box<Node<T>>>,
        key: i32,
        op: fn(&T, &T) -> T,
        e: &T,
    ) -> Option<Box<Node<T>>> {
        match node {
            None => None,
            Some(mut n) => {
                n.left = Self::erase_all(n.left.take(), key, op, e);
                n.right = Self::erase_all(n.right.take(), key, op, e);
                if n.key == key {
                    return Self::merge(n.left.take(), n.right.take(), op, e);
                }
                n.update(op, e);
                Some(n)
            }
        }
    }

    fn merge(
        a: Option<Box<Node<T>>>,
        b: Option<Box<Node<T>>>,
        op: fn(&T, &T) -> T,
        e: &T,
    ) -> Option<Box<Node<T>>> {
        match (a, b) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut lnode), Some(mut rnode)) => {
                if lnode.prio > rnode.prio {
                    lnode.right = Self::merge(lnode.right.take(), Some(rnode), op, e);
                    lnode.update(op, e);
                    Some(lnode)
                } else {
                    rnode.left = Self::merge(Some(lnode), rnode.left.take(), op, e);
                    rnode.update(op, e);
                    Some(rnode)
                }
            }
        }
    }

    // keyで分割。key未満を左、key以上を右にする
    fn split(
        node: Option<Box<Node<T>>>,
        key: i32,
        op: fn(&T, &T) -> T,
        e: &T,
    ) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        match node {
            None => (None, None),
            Some(mut n) => {
                if n.key < key {
                    let (l, r) = Self::split(n.right.take(), key, op, e);
                    n.right = l;
                    n.update(op, e);
                    (Some(n), r)
                } else {
                    let (l, r) = Self::split(n.left.take(), key, op, e);
                    n.left = r;
                    n.update(op, e);
                    (l, Some(n))
                }
            }
        }
    }
}

pub struct ImplicitTreap<T: Clone + std::fmt::Debug> {
    root: Option<Box<Node<T>>>,
    rng: XorShift32,
    op: fn(&T, &T) -> T,
    e: T,
}

impl<T: Clone + std::fmt::Debug> ImplicitTreap<T> {
    pub fn new(op: fn(&T, &T) -> T, e: T) -> Self {
        ImplicitTreap {
            root: None,
            rng: XorShift32::new(0x12345678),
            op,
            e,
        }
    }

    pub fn insert(&mut self, key: i32, val: T) {
        self.root = Node::insert(self.root.take(), key, val, &mut self.rng, self.op, &self.e);
    }

    pub fn erase(&mut self, key: i32) {
        self.root = Node::erase_all(self.root.take(), key, self.op, &self.e);
    }

    pub fn update(&mut self, key: i32, val: T) {
        self.erase(key);
        self.insert(key, val);
    }

    pub fn prod(&mut self, l: i32, r: i32) -> T {
        let (t1, t2) = Node::split(self.root.take(), l, self.op, &self.e);
        let (t21, t22) = Node::split(t2, r, self.op, &self.e);

        let res = t21.as_ref().map_or(self.e.clone(), |n| n.sum.clone());

        let merged = Node::merge(t21, t22, self.op, &self.e);
        self.root = Node::merge(t1, merged, self.op, &self.e);

        res
    }

    pub fn all_prod(&self) -> T {
        self.root.as_ref().map_or(self.e.clone(), |n| n.sum.clone())
    }

    pub fn split(mut self, key: i32) -> (Self, Self) {
        let (left, right) = Node::split(self.root.take(), key, self.op, &self.e);
        (
            ImplicitTreap {
                root: left,
                rng: XorShift32::new(self.rng.state),
                op: self.op,
                e: self.e.clone(),
            },
            ImplicitTreap {
                root: right,
                rng: XorShift32::new(self.rng.state),
                op: self.op,
                e: self.e.clone(),
            },
        )
    }

    pub fn merge(mut self, mut other: Self) -> Self {
        let root = Node::merge(self.root.take(), other.root.take(), self.op, &self.e);
        ImplicitTreap {
            root,
            rng: XorShift32::new(self.rng.state ^ other.rng.state),
            op: self.op,
            e: self.e.clone(),
        }
    }
}

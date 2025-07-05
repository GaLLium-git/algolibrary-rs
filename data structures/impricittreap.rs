fn main() {
    let mut treap = ImplicitTreap::new(|&a, &b| a + b, 0);

    treap.insert(10, 1);
    treap.insert(20, 2);
    treap.insert(15, 3);
    treap.insert(25, 4);

    println!("All sum: {}", treap.all_prod()); // 10

    println!("prod(10, 20): {}", treap.prod(10, 20)); // 4 (1 + 3)
    println!("prod(15, 30): {}", treap.prod(15, 30)); // 9 (3 + 2 + 4)

    println!("Size: {}", treap.size()); // 4

    println!("kth(0): {:?}", treap.kth(0)); // Some(1)
    println!("kth(1): {:?}", treap.kth(1)); // Some(3)
    println!("kth(2): {:?}", treap.kth(2)); // Some(2)
    println!("kth(3): {:?}", treap.kth(3)); // Some(4)
    println!("kth(4): {:?}", treap.kth(4)); // None

    println!("get(15): {:?}", treap.get(15)); // Some(3)
    println!("get(30): {:?}", treap.get(30)); // None

    treap.update(15, 10); // 3 -> 10
    println!("After update: prod(10,30): {}", treap.prod(10, 30)); // 1 + 10 + 2 + 4 = 17
    println!("Size after update: {}", treap.size()); // 4
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
    size: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Clone + std::fmt::Debug> Node<T> {
    fn new(key: i32, val: T, prio: u32, _op: fn(&T, &T) -> T) -> Box<Self> {
        Box::new(Node {
            key,
            val: val.clone(),
            sum: val,
            prio,
            size: 1,
            left: None,
            right: None,
        })
    }

    fn update(&mut self, op: fn(&T, &T) -> T, e: &T) {
        self.sum = self.val.clone();
        self.size = 1;
        if let Some(l) = &self.left {
            self.sum = op(&l.sum, &self.sum);
            self.size += l.size;
        }
        if let Some(r) = &self.right {
            self.sum = op(&self.sum, &r.sum);
            self.size += r.size;
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

    fn kth(&self, k: usize) -> Option<&T> {
        let lsize = self.left.as_ref().map_or(0, |l| l.size);
        if k < lsize {
            self.left.as_ref().and_then(|l| l.kth(k))
        } else if k == lsize {
            Some(&self.val)
        } else {
            self.right.as_ref().and_then(|r| r.kth(k - lsize - 1))
        }
    }

    fn get(&self, key: i32) -> Option<&T> {
        if self.key == key {
            Some(&self.val)
        } else if key < self.key {
            self.left.as_ref().and_then(|l| l.get(key))
        } else {
            self.right.as_ref().and_then(|r| r.get(key))
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

    pub fn size(&self) -> usize {
        self.root.as_ref().map_or(0, |n| n.size)
    }

    pub fn kth(&self, k: usize) -> Option<&T> {
        if k >= self.size() {
            None
        } else {
            self.root.as_ref().and_then(|node| node.kth(k))
        }
    }

    pub fn get(&self, key: i32) -> Option<&T> {
        self.root.as_ref().and_then(|node| node.get(key))
    }
}


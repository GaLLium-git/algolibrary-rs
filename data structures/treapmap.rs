fn main() {
    fn op(a: &i32, b: &i32) -> i32 {
        a + b
    }

    let mut treap = TreapMap::new(op, 0);

    treap.insert(3, 10);
    treap.insert(1, 20);
    treap.insert(5, 30);
    treap.insert(2, 15);

    println!("All sum = {}", treap.all_prod()); // 75

    println!("Get key=3: {:?}", treap.get(3)); // Some(10)
    println!("Get key=4: {:?}", treap.get(4)); // None

    for k in 0..treap.size() {
        println!("kth({}) = {:?}", k, treap.kth(k));
    }

    println!("Prod [2, 5) = {}", treap.prod(2, 5)); // 25

    treap.erase(3);
    println!("After erase key=3, all sum = {}", treap.all_prod()); // 65
    treap.debug();

    treap.update(1, 100);
    println!("After update key=1 to 100, get(1) = {:?}", treap.get(1)); // Some(100)
    println!("New all sum = {}", treap.all_prod()); // 145
    
    println!("\n--- Split Test ---");
    let mut split_test =TreapMap::new(op, 0);
    split_test.insert(0, 1);
    split_test.insert(1, 2);
    split_test.insert(2, 3);
    split_test.insert(3, 4);
    split_test.insert(4, 5);

    let (left, right) = split_test.split(3);
    println!("Left sum = {}", left.all_prod()); // 1+2+3 = 6
    println!("Right sum = {}", right.all_prod()); // 4+5 = 9

    println!("Left contents:");
    for i in 0..left.size() {
        println!("  kth({}) = {:?}", i, left.kth(i));
    }

    println!("Right contents:");
    for i in 0..right.size() {
        println!("  kth({}) = {:?}", i, right.kth(i));
    }

    // === Merge Test ===
    println!("\n--- Merge Test ---");
    let merged = left.merge(right);
    println!("Merged sum = {}", merged.all_prod()); // 15
    println!("Merged contents:");
    for i in 0..merged.size() {
        println!("  kth({}) = {:?}", i, merged.kth(i));
    }
}




// === Treap本体 ===


#[derive(Clone)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        XorShift64 { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}


#[derive(Debug)]
struct Node<K:Copy + std::fmt::Debug + std::cmp::PartialOrd + std::cmp::PartialEq + std::fmt::Display, T: Clone + std::fmt::Debug> {
    key: K,
    val: T,
    sum: T,
    prio: u64,
    size: usize,
    left: Option<Box<Node<K,T>>>,
    right: Option<Box<Node<K,T>>>,
}

impl<K: Copy + std::fmt::Debug + std::cmp::PartialOrd + std::cmp::PartialEq + std::fmt::Display, T: Clone + std::fmt::Debug> Node<K,T> {
    fn new(key: K, val: T, prio: u64, _op: fn(&T, &T) -> T) -> Box<Self> {
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

    fn update(&mut self, op: fn(&T, &T) -> T, _e: &T) {
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

    fn rotate_left(mut node: Box<Node<K,T>>, op: fn(&T, &T) -> T, e: &T) -> Box<Node<K,T>> {
        let mut new_root = node.right.take().unwrap();
        node.right = new_root.left.take();
        node.update(op, e);
        new_root.left = Some(node);
        new_root.update(op, e);
        new_root
    }

    fn rotate_right(mut node: Box<Node<K,T>>, op: fn(&T, &T) -> T, e: &T) -> Box<Node<K,T>> {
        let mut new_root = node.left.take().unwrap();
        node.left = new_root.right.take();
        node.update(op, e);
        new_root.right = Some(node);
        new_root.update(op, e);
        new_root
    }

    fn insert(
        node: Option<Box<Node<K,T>>>,
        key: K,
        val: T,
        rng: &mut XorShift64,
        op: fn(&T, &T) -> T,
        e: &T,
    ) -> Option<Box<Node<K,T>>> {
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

    fn merge(
        a: Option<Box<Node<K,T>>>,
        b: Option<Box<Node<K,T>>>,
        op: fn(&T, &T) -> T,
        e: &T,
    ) -> Option<Box<Node<K,T>>> {
        match (a, b) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.prio > r.prio {
                    l.right = Self::merge(l.right.take(), Some(r), op, e);
                    l.update(op, e);
                    Some(l)
                } else {
                    r.left = Self::merge(Some(l), r.left.take(), op, e);
                    r.update(op, e);
                    Some(r)
                }
            }
        }
    }

    fn split(
        node: Option<Box<Node<K,T>>>,
        key: K,
        op: fn(&T, &T) -> T,
        e: &T,
    ) -> (Option<Box<Node<K,T>>>, Option<Box<Node<K,T>>>) {
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

    fn get(&self, key: K) -> Option<&T> {
        if self.key == key {
            Some(&self.val)
        } else if key < self.key {
            self.left.as_ref().and_then(|l| l.get(key))
        } else {
            self.right.as_ref().and_then(|r| r.get(key))
        }
    }

    fn next_key(&self, key: K) -> Option<K> {
        if self.key <= key {
            self.right.as_ref()?.next_key(key)
        } else {
            match &self.left {
                Some(left) => left.next_key(key).or(Some(self.key)),
                None => Some(self.key),
            }
        }
    }    
}

pub struct TreapMap<K: Copy + std::fmt::Debug + std::cmp::PartialOrd + std::cmp::PartialEq +std::fmt::Display, T: Clone + std::fmt::Debug> {
    root: Option<Box<Node<K,T>>>,
    rng: XorShift64,
    op: fn(&T, &T) -> T,
    e: T,
}

impl<K: Copy + std::fmt::Debug + std::cmp::PartialOrd + std::cmp::PartialEq + std::fmt::Display, T: Clone + std::fmt::Debug> TreapMap<K,T> {
    pub fn new(op: fn(&T, &T) -> T, e: T) -> Self {
        Self {
            root: None,
            rng: XorShift64::new(0x12345678),
            op,
            e,
        }
    }

    pub fn size(&self) -> usize {
        self.root.as_ref().map_or(0, |n| n.size)
    }

    pub fn insert(&mut self, key: K, val: T) {
        self.root = Node::insert(self.root.take(), key, val, &mut self.rng, self.op, &self.e);
    }

    pub fn erase(&mut self, key: K) {
        // Efficient O(logN) erase using split and merge
        let (left, mid_right) = Node::split(self.root.take(), key, self.op, &self.e);
        let (mid, right) = if let Some(next)= mid_right.as_ref().and_then(|n| n.next_key(key)) {Node::split(mid_right, next, self.op, &self.e)} else {(mid_right,None)};
        self.root = Node::merge(left, right, self.op, &self.e);
    }

    pub fn update(&mut self, key: K, val: T) {
        self.erase(key);
        self.insert(key, val);
    }

    pub fn get(&self, key: K) -> Option<&T> {
        self.root.as_ref().and_then(|n| n.get(key))
    }

    pub fn kth(&self, k: usize) -> Option<&T> {
        if k >= self.size() {
            None
        } else {
            self.root.as_ref().and_then(|n| n.kth(k))
        }
    }

    pub fn prod(&mut self, l: K, r: K) -> T {
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

    pub fn split(self, key: K) -> (Self, Self) {
        let (left, right) = Node::split(self.root, key, self.op, &self.e);
        (
            Self {
                root: left,
                rng: self.rng.clone(),
                op: self.op,
                e: self.e.clone(),
            },
            Self {
                root: right,
                rng: self.rng.clone(),
                op: self.op,
                e: self.e.clone(),
            },
        )
    }

    pub fn merge(self, other: Self) -> Self {
        let root = Node::merge(self.root, other.root, self.op, &self.e);
        Self {
            root,
            rng: self.rng,
            op: self.op,
            e: self.e.clone(),
        }
    }

    pub fn debug(&self) {
        fn inorder<K: Copy + std::fmt::Debug + std::cmp::PartialOrd + std::cmp::PartialEq+std::fmt::Display, T: Clone + std::fmt::Debug>(
            node: &Option<Box<Node<K,T>>>,
            out: &mut Vec<(K, T)>,
        ) {
            if let Some(n) = node {
                inorder(&n.left, out);
                out.push((n.key, n.val.clone()));
                inorder(&n.right, out);
            }
        }

        let mut res = vec![];
        inorder(&self.root, &mut res);

        for (i, (k, v)) in res.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("({},{:?})", k, v);
        }
        println!();
    }
}


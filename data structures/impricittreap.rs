fn main() {
    fn op(a: &i32, b: &i32) -> i32 {
        a + b
    }

    let mut treap = ImplicitTreap::new(op, 0);

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

    println!("Prod [2, 5) = {}", treap.prod(2, 5)); // 55

    treap.erase(3);
    println!("After erase key=3, all sum = {}", treap.all_prod()); // 65

    treap.update(1, 100);
    println!("After update key=1 to 100, get(1) = {:?}", treap.get(1)); // Some(100)
    println!("New all sum = {}", treap.all_prod()); // 145
    
    println!("\n--- Split Test ---");
    let mut split_test = ImplicitTreap::new(op, 0);
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

pub struct ImplicitTreap<T: Clone + std::fmt::Debug> {
    root: Option<Box<Node<T>>>,
    rng: XorShift64,
    op: fn(&T, &T) -> T,
    e: T,
}

impl<T: Clone + std::fmt::Debug> ImplicitTreap<T> {
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

    pub fn insert(&mut self, key: usize, val: T) {
        self.root = Node::insert(self.root.take(), key, val, &mut self.rng, self.op, &self.e);
    }

    pub fn erase(&mut self, key: usize) {
        // Efficient O(logN) erase using split and merge
        let (left, mid_right) = Node::split(self.root.take(), key, self.op, &self.e);
        let (mid, right) = Node::split(mid_right, key + 1, self.op, &self.e);
        self.root = Node::merge(left, right, self.op, &self.e);
    }

    pub fn update(&mut self, key: usize, val: T) {
        self.erase(key);
        self.insert(key, val);
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        self.root.as_ref().and_then(|n| n.get(key))
    }

    pub fn kth(&self, k: usize) -> Option<&T> {
        if k >= self.size() {
            None
        } else {
            self.root.as_ref().and_then(|n| n.kth(k))
        }
    }

    pub fn prod(&mut self, l: usize, r: usize) -> T {
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

    pub fn split(self, key: usize) -> (Self, Self) {
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
        fn inorder<T: Clone + std::fmt::Debug>(
            node: &Option<Box<Node<T>>>,
            out: &mut Vec<(usize, T)>,
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


 



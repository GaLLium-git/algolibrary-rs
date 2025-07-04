fn main() {
    // 例: 遅延伝搬区間和
    fn op(a: &i64, b: &i64) -> i64 {
        a + b
    }
    fn mapping(f: &i64, x: &i64) -> i64 {
        f + x
    }
    fn composite(f: &i64, g: &i64) -> i64 {
        f + g
    }
    let id = 0i64;
    let e = 0i64;

    let mut treap = LazyImplicitTreap::new(op, e, mapping, id, composite);

    treap.insert(1, 10);
    treap.insert(8, 30);
    treap.insert(8, 20);
    treap.insert(16,100);
    treap.debug(); // (1,10) (2,20) (3,30)

    treap.erase(8);
    treap.debug(); // (1,10) (3,30)

    treap.update(3, 100);
    treap.debug(); // (1,10) (3,100)

    println!("prod(1,4) = {}", treap.prod(1, 4)); // 10+100=110

    treap.apply(1, 4, 5);
    treap.debug(); // (1,15) (3,105)

    println!("all_prod = {}", treap.all_prod()); // 15 + 105 = 120
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
struct Node<T: Clone + std::fmt::Debug , F: Clone +std::fmt::Debug + PartialEq> {
    key: i32,
    val: T,
    sum: T,
    prio: u32,
    lazy: F,
    left: Option<Box<Node<T, F>>>,
    right: Option<Box<Node<T, F>>>,
}

impl<T: Clone + std::fmt::Debug , F: Clone + std::fmt::Debug + PartialEq> Node<T, F> {
    fn new(val: T, prio: u32, e: &T, id: &F, key: i32) -> Box<Self> {
        Box::new(Node {
            key,
            val: val.clone(),
            sum: val,
            prio,
            lazy: id.clone(),
            left: None,
            right: None,
        })
    }

    fn update(&mut self, op: fn(&T, &T) -> T, e: &T) {
        self.sum = self.val.clone();
        if let Some(l) = &self.left {
            self.sum = op(&l.sum, &self.sum);
        }
        if let Some(r) = &self.right {
            self.sum = op(&self.sum, &r.sum);
        }
    }

    fn apply(&mut self, mapping: fn(&F, &T) -> T, composite: fn(&F, &F) -> F, f: &F) {
        self.val = mapping(f, &self.val);
        self.sum = mapping(f, &self.sum);
        self.lazy = composite(f, &self.lazy);
    }

    fn push_down(
        &mut self,
        mapping: fn(&F, &T) -> T,
        composite: fn(&F, &F) -> F,
        id: &F,
    ) {
        if self.lazy != *id {
            if let Some(l) = self.left.as_mut() {
                l.apply(mapping, composite, &self.lazy);
            }
            if let Some(r) = self.right.as_mut() {
                r.apply(mapping, composite, &self.lazy);
            }
            self.lazy = id.clone();
        }
    }

    fn rotate_left(
        mut node: Box<Node<T, F>>,
        op: fn(&T, &T) -> T,
        e: &T,
        mapping: fn(&F, &T) -> T,
        composite: fn(&F, &F) -> F,
        id: &F,
    ) -> Box<Node<T, F>> {
        let mut new_root = node.right.take().unwrap();
        node.right = new_root.left.take();
        node.update(op, e);
        new_root.left = Some(node);
        new_root.update(op, e);
        new_root.lazy = id.clone();
        new_root
    }

    fn rotate_right(
        mut node: Box<Node<T, F>>,
        op: fn(&T, &T) -> T,
        e: &T,
        mapping: fn(&F, &T) -> T,
        composite: fn(&F, &F) -> F,
        id: &F,
    ) -> Box<Node<T, F>> {
        let mut new_root = node.left.take().unwrap();
        node.left = new_root.right.take();
        node.update(op, e);
        new_root.right = Some(node);
        new_root.update(op, e);
        new_root.lazy = id.clone();
        new_root
    }

    // insert always goes right if key >= node.key (to allow duplicates)
    fn insert(
        node: Option<Box<Node<T, F>>>,
        key: i32,
        val: T,
        rng: &mut XorShift32,
        op: fn(&T, &T) -> T,
        mapping: fn(&F, &T) -> T,
        composite: fn(&F, &F) -> F,
        id: &F,
        e: &T,
    ) -> Option<Box<Node<T, F>>> {
        match node {
            None => Some(Self::new(val, rng.next(), e, id, key)),
            Some(mut n) => {
                n.push_down(mapping, composite, id);
                if key < n.key {
                    n.left = Self::insert(n.left.take(), key, val, rng, op, mapping, composite, id, e);
                    if n.left.as_ref().unwrap().prio > n.prio {
                        return Some(Self::rotate_right(n, op, e, mapping, composite, id));
                    }
                } else {
                    // key >= n.key, go right (allow duplicates)
                    n.right = Self::insert(n.right.take(), key, val, rng, op, mapping, composite, id, e);
                    if n.right.as_ref().unwrap().prio > n.prio {
                        return Some(Self::rotate_left(n, op, e, mapping, composite, id));
                    }
                }
                n.update(op, e);
                Some(n)
            }
        }
    }

    
    fn erase_all(
        node: Option<Box<Node<T, F>>>,
        key: i32,
        op: fn(&T, &T) -> T,
        e: &T,
        mapping: fn(&F, &T) -> T,
        composite: fn(&F, &F) -> F,
        id: &F,
    ) -> Option<Box<Node<T, F>>> {
        // 1. keyより小さい部分木, それ以外に分割
        let (less, geq) = Self::split(node, key, op, e, mapping, composite, id);
        // 2. key以上の部分木をさらにkey+1で分割し、keyに等しい部分木を切り離す
        let (eq, greater) = Self::split(geq, key + 1, op, e, mapping, composite, id);
        // eq は key == target の部分木なので破棄、less と greater をマージ
        Self::merge(less, greater, op, e, mapping, composite, id)
    }


    fn merge(
        a: Option<Box<Node<T, F>>>,
        b: Option<Box<Node<T, F>>>,
        op: fn(&T, &T) -> T,
        e: &T,
        mapping: fn(&F, &T) -> T,
        composite: fn(&F, &F) -> F,
        id: &F,
    ) -> Option<Box<Node<T, F>>> {
        match (a, b) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut lnode), Some(mut rnode)) => {
                lnode.push_down(mapping, composite, id);
                rnode.push_down(mapping, composite, id);
                if lnode.prio > rnode.prio {
                    lnode.right = Self::merge(lnode.right.take(), Some(rnode), op, e, mapping, composite, id);
                    lnode.update(op, e);
                    Some(lnode)
                } else {
                    rnode.left = Self::merge(Some(lnode), rnode.left.take(), op, e, mapping, composite, id);
                    rnode.update(op, e);
                    Some(rnode)
                }
            }
        }
    }

    fn split(
        node: Option<Box<Node<T, F>>>,
        key: i32,
        op: fn(&T, &T) -> T,
        e: &T,
        mapping: fn(&F, &T) -> T,
        composite: fn(&F, &F) -> F,
        id: &F,
    ) -> (Option<Box<Node<T, F>>>, Option<Box<Node<T, F>>>) {
        match node {
            None => (None, None),
            Some(mut n) => {
                n.push_down(mapping, composite, id);
                if n.key < key {
                    let (l, r) = Self::split(n.right.take(), key, op, e, mapping, composite, id);
                    n.right = l;
                    n.update(op, e);
                    (Some(n), r)
                } else {
                    let (l, r) = Self::split(n.left.take(), key, op, e, mapping, composite, id);
                    n.left = r;
                    n.update(op, e);
                    (l, Some(n))
                }
            }
        }
    }

    fn apply_range(
        node: Option<Box<Node<T, F>>>,
        l: i32,
        r: i32,
        mapping: fn(&F, &T) -> T,
        composite: fn(&F, &F) -> F,
        id: &F,
        f: &F,
        op: fn(&T, &T) -> T,
        e: &T,
    ) -> Option<Box<Node<T, F>>> {
        if node.is_none() || l >= r {
            return node;
        }
        let mut root = node.unwrap();

        // split into <l, [l,r), >=r
        let (left, rest) = Self::split(Some(root), l, op, e, mapping, composite, id);
        let (mid, right) = Self::split(rest, r, op, e, mapping, composite, id);

        let mid = mid.map(|mut m| {
            m.apply(mapping, composite, f);
            m
        });

        // merge back
        let merged = Self::merge(left, mid, op, e, mapping, composite, id);
        Self::merge(merged, right, op, e, mapping, composite, id)
    }

    fn prod_range(
        node: &Option<Box<Node<T, F>>>,
        l: i32,
        r: i32,
        op: fn(&T, &T) -> T,
        e: &T,
    ) -> T {
        if node.is_none() || l >= r {
            return e.clone();
        }
        let n = node.as_ref().unwrap();
        if n.key >= r {
            return Self::prod_range(&n.left, l, r, op, e);
        }
        if n.key < l {
            return Self::prod_range(&n.right, l, r, op, e);
        }
        let mut res = e.clone();
        // left subtree contribution
        res = op(&res, &Self::prod_range(&n.left, l, r, op, e));
        // current node contribution if in range
        if l <= n.key && n.key < r {
            res = op(&res, &n.val);
        }
        // right subtree contribution
        res = op(&res, &Self::prod_range(&n.right, l, r, op, e));
        res
    }

    fn inorder(node: &Option<Box<Node<T, F>>>, out: &mut Vec<(i32, T)>) {
        if let Some(n) = node {
            Self::inorder(&n.left, out);
            out.push((n.key, n.val.clone()));
            Self::inorder(&n.right, out);
        }
    }

    fn update_key_val(
        node: &mut Option<Box<Node<T, F>>>,
        key: i32,
        val: T,
        op: fn(&T, &T) -> T,
        e: &T,
        mapping: fn(&F, &T) -> T,
        composite: fn(&F, &F) -> F,
        id: &F,
    ) -> bool {
        if let Some(n) = node {
            n.push_down(mapping, composite, id);
            if key == n.key {
                n.val = val;
                n.update(op, e);
                true
            } else if key < n.key {
                let updated = Self::update_key_val(&mut n.left, key, val.clone(), op, e, mapping, composite, id);
                if updated {
                    n.update(op, e);
                }
                updated
            } else {
                let updated = Self::update_key_val(&mut n.right, key, val.clone(), op, e, mapping, composite, id);
                if updated {
                    n.update(op, e);
                }
                updated
            }
        } else {
            false
        }
    }
}

pub struct LazyImplicitTreap<T: Clone + std::fmt::Debug , F: Clone +std::fmt::Debug+ PartialEq> {
    root: Option<Box<Node<T, F>>>,
    rng: XorShift32,
    op: fn(&T, &T) -> T,
    e: T,
    mapping: fn(&F, &T) -> T,
    id: F,
    composite: fn(&F, &F) -> F,
}

impl<T: Clone + std::fmt::Debug, F: Clone + std::fmt::Debug + PartialEq> LazyImplicitTreap<T, F> {
    pub fn new(
        op: fn(&T, &T) -> T,
        e: T,
        mapping: fn(&F, &T) -> T,
        id: F,
        composite: fn(&F, &F) -> F,
    ) -> Self {
        LazyImplicitTreap {
            root: None,
            rng: XorShift32::new(0x12345678),
            op,
            e,
            mapping,
            id,
            composite,
        }
    }

    pub fn insert(&mut self, key: i32, val: T) {
        self.root = Node::insert(
            self.root.take(),
            key,
            val,
            &mut self.rng,
            self.op,
            self.mapping,
            self.composite,
            &self.id,
            &self.e,
        );
    }

    pub fn erase(&mut self, key: i32) {
        self.root = Node::erase_all(
            self.root.take(),
            key,
            self.op,
            &self.e,
            self.mapping,
            self.composite,
            &self.id,
        );
    }

    pub fn update(&mut self, key: i32, val: T) {
        self.erase(key);
        self.insert(key,val);
    }

    pub fn prod(&self, l: i32, r: i32) -> T {
        Node::prod_range(&self.root, l, r, self.op, &self.e)
    }

    pub fn apply(&mut self, l: i32, r: i32, f: F) {
        self.root = Node::apply_range(
            self.root.take(),
            l,
            r,
            self.mapping,
            self.composite,
            &self.id,
            &f,
            self.op,
            &self.e,
        );
    }

    pub fn all_prod(&self) -> T {
        if let Some(r) = &self.root {
            r.sum.clone()
        } else {
            self.e.clone()
        }
    }

    pub fn debug(&self) {
        let mut out = Vec::new();
        Node::inorder(&self.root, &mut out);
        for (k, v) in out {
            print!("({},{:?}) ", k, v);
        }
        println!();
    }
}

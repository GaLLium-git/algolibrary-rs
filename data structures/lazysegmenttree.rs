#[derive(Clone, Debug, PartialEq)]
struct Node {
    sum: i64,
    len: i64,
}

fn main() {
    let n = 8;
    let op = |a: &Node, b: &Node| Node {
        sum: a.sum + b.sum,
        len: a.len + b.len,
    };
    let e = Node { sum: 0, len: 0 };
    let mapping = |f: &i64, s: &Node| Node {
        sum: s.sum + (*f) * s.len,
        len: s.len,
    };
    let composition = |f: &i64, g: &i64| *f + *g;
    let id = 0i64;

    let mut seg = LazySegTree::new(n, op, e, mapping, composition, id);
    for i in 0..n {
        seg.set(i, Node { sum: 1, len: 1 });
    }

    println!("{}", seg.prod(0, 8).sum); // 8
    seg.apply(2, 6, 10);
    println!("{}", seg.get(3).sum); // 11
    println!("{}", seg.prod(2, 6).sum); // 44
    let r = seg.max_right(0, |s: &Node| s.sum <= 20);
    println!("r = {}", r);
    let l = seg.min_left(r, |s: &Node| s.sum <= 20);
    println!("l = {}", l);
}


#[derive(Clone)]
pub struct LazySegTree<S, F, Op, Mapping, Composition>
where
    S: Clone,
    F: Clone,
    Op: Fn(&S, &S) -> S,
    Mapping: Fn(&F, &S) -> S,
    Composition: Fn(&F, &F) -> F,
{
    n: usize,
    size: usize,
    data: Vec<S>,
    lazy: Vec<F>,
    op: Op,
    e: S,
    mapping: Mapping,
    composition: Composition,
    id: F,
    log: u32,
}

impl<S, F, Op, Mapping, Composition> LazySegTree<S, F, Op, Mapping, Composition>
where
    S: Clone,
    F: Clone + std::cmp::PartialEq,
    Op: Fn(&S, &S) -> S,
    Mapping: Fn(&F, &S) -> S,
    Composition: Fn(&F, &F) -> F,
{
    pub fn new(size: usize, op: Op, e: S, mapping: Mapping, composition: Composition, id: F) -> Self {
        let mut n = 1usize;
        let mut log = 0u32;
        while n < size {
            n <<= 1;
            log += 1;
        }
        Self {
            n,
            size,
            data: vec![e.clone(); 2 * n],
            lazy: vec![id.clone(); 2 * n],
            op,
            e,
            mapping,
            composition,
            id,
            log,
        }
    }

    pub fn set(&mut self, mut i: usize, x: S) {
        i += self.n;
        self.push_path(i);
        self.data[i] = x;
        self.pull_path(i);
    }

    pub fn get(&mut self, mut i: usize) -> S {
        i += self.n;
        self.push_path(i);
        self.data[i].clone()
    }

    pub fn all_prod(&self) -> S {
        self.data[1].clone()
    }

    pub fn prod(&mut self, mut l: usize, mut r: usize) -> S {
        if l == r {
            return self.e.clone();
        }
        l += self.n;
        r += self.n;
        self.push_path(l);
        self.push_path(r - 1);
        let mut sml = self.e.clone();
        let mut smr = self.e.clone();
        while l < r {
            if l & 1 == 1 {
                sml = (self.op)(&sml, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = (self.op)(&self.data[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&sml, &smr)
    }

    pub fn apply(&mut self, mut l: usize, mut r: usize, f: F) {
        if l == r {
            return;
        }
        l += self.n;
        r += self.n;
        self.push_path(l);
        self.push_path(r - 1);
        let (l0, r0) = (l, r);
        while l < r {
            if l & 1 == 1 {
                self.all_apply(l, &f);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.all_apply(r, &f);
            }
            l >>= 1;
            r >>= 1;
        }
        self.rebuild_on_range(l0, r0);
    }

    pub fn apply_at(&mut self, mut i: usize, f: F) {
        i += self.n;
        self.push_path(i);
        self.all_apply(i, &f);
        self.pull_path(i);
    }

    pub fn max_right<G>(&mut self, mut l: usize, pred: G) -> usize
    where
        G: Fn(&S) -> bool,
    {
        if l == self.size {
            return self.size;
        }
        l += self.n;
        self.push_path(l);
        let mut sm = self.e.clone();
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            let nxt = (self.op)(&sm, &self.data[l]);
            if !pred(&nxt) {
                while l < self.n {
                    self.push(l);
                    l <<= 1;
                    let t = (self.op)(&sm, &self.data[l]);
                    if pred(&t) {
                        sm = t;
                        l += 1;
                    }
                }
                return l - self.n;
            }
            sm = nxt;
            l += 1;
            if (l & (l - 1)) == 0 {
                break;
            }
        }
        self.size
    }

    pub fn min_left<G>(&mut self, mut r: usize, pred: G) -> usize
    where
        G: Fn(&S) -> bool,
    {
        if r == 0 {
            return 0;
        }
        r += self.n;
        self.push_path(r - 1);
        let mut sm = self.e.clone();
        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            let nxt = (self.op)(&self.data[r], &sm);
            if !pred(&nxt) {
                while r < self.n {
                    self.push(r);
                    r = r * 2 + 1;
                    let t = (self.op)(&self.data[r], &sm);
                    if pred(&t) {
                        sm = t;
                        r -= 1;
                    }
                }
                return r + 1 - self.n;
            }
            sm = nxt;
            if (r & (r - 1)) == 0 {
                break;
            }
        }
        0
    }

    fn all_apply(&mut self, k: usize, f: &F) {
        self.data[k] = (self.mapping)(f, &self.data[k]);
        if k < self.n {
            self.lazy[k] = (self.composition)(f, &self.lazy[k]);
        }
    }

    fn push(&mut self, k: usize) {
        let f = self.lazy[k].clone();
        if f != self.id {
            if k < self.n {
                let lc = k << 1;
                let rc = lc | 1;
                self.all_apply(lc, &f);
                self.all_apply(rc, &f);
            }
            self.lazy[k] = self.id.clone();
        }
    }

    fn update(&mut self, k: usize) {
        self.data[k] = (self.op)(&self.data[k << 1], &self.data[k << 1 | 1]);
    }

    fn pull_path(&mut self, mut k: usize) {
        for _ in 0..self.log {
            k >>= 1;
            self.update(k);
            if k == 1 {
                break;
            }
        }
    }

    fn push_path(&mut self, mut k: usize) {
        let mut stack: Vec<usize> = Vec::with_capacity(self.log as usize);
        let mut x = k >> 1;
        for _ in 0..self.log {
            if x == 0 {
                break;
            }
            stack.push(x);
            x >>= 1;
        }
        for &v in stack.iter().rev() {
            self.push(v);
        }
    }

    fn rebuild_on_range(&mut self, l0: usize, r0: usize) {
        for i in 1..=self.log {
            if ((l0 >> i) << i) != l0 {
                self.update(l0 >> i);
            }
            if ((r0 >> i) << i) != r0 {
                self.update((r0 - 1) >> i);
            }
        }
    }
}

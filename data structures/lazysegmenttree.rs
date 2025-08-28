fn main() {
    let op = |a: &i64, b: &i64| a + b;
    let e = 0i64;
    let mapping = |x: &i64, f: &i64, len: usize| x + f * len as i64;
    let composition = |f: &i64, g: &i64| f + g;
    let id = 0i64;

    let mut seg = LazySegtree::new(8, op, e, mapping, composition, id);

    for i in 0..8 {
        seg.set(i, i as i64);
    }

    println!("sum(0..8) = {}", seg.prod(0, 8)); // 28

    seg.apply_range(2, 6, 10);

    println!("sum(0..8) = {}", seg.prod(0, 8)); // 28 + 40 = 68
    println!("get(3) = {}", seg.get(3)); // 13
}

pub struct LazySegtree<S, F, G, H>
where
    S: Clone,
    F: Clone,
    G: Fn(&S, &S) -> S,
    H: Fn(&S, &F, usize) -> S, 
{
    n: usize,
    size: usize,
    data: Vec<S>,
    lazy: Vec<F>,
    op: G,
    e: S,
    mapping: H,
    composition: fn(&F, &F) -> F,
    id: F,
}

impl<S, F, G, H> LazySegtree<S, F, G, H>
where
    S: Clone,
    F: Clone + std::cmp::PartialEq,
    G: Fn(&S, &S) -> S,
    H: Fn(&S, &F, usize) -> S,
{
    pub fn new(size: usize, op: G, e: S, mapping: H, composition: fn(&F, &F) -> F, id: F) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        LazySegtree {
            n,
            size,
            data: vec![e.clone(); 2 * n],
            lazy: vec![id.clone(); 2 * n],
            op,
            e,
            mapping,
            composition,
            id,
        }
    }

    fn push(&mut self, k: usize, len: usize) {
        if self.lazy[k] != self.id {
            let f = self.lazy[k].clone();
            self.all_apply(k << 1, &f, len >> 1);
            self.all_apply(k << 1 | 1, &f, len >> 1);
            self.lazy[k] = self.id.clone();
        }
    }

    fn all_apply(&mut self, k: usize, f: &F, len: usize) {
        self.data[k] = (self.mapping)(&self.data[k], f, len);
        if k < self.n {
            self.lazy[k] = (self.composition)(f, &self.lazy[k]);
        }
    }

    fn update(&mut self, k: usize) {
        self.data[k] = (self.op)(&self.data[k << 1], &self.data[k << 1 | 1]);
    }

    pub fn set(&mut self, mut p: usize, x: S) {
        p += self.n;
        for i in (1..=self.log()).rev() {
            self.push(p >> i, 1 << i);
        }
        self.data[p] = x;
        for i in 1..=self.log() {
            self.update(p >> i);
        }
    }

    pub fn get(&mut self, mut p: usize) -> S {
        p += self.n;
        for i in (1..=self.log()).rev() {
            self.push(p >> i, 1 << i);
        }
        self.data[p].clone()
    }

    pub fn prod(&mut self, mut l: usize, mut r: usize) -> S {
        if l == r {
            return self.e.clone();
        }
        l += self.n;
        r += self.n;
        for i in (1..=self.log()).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i, 1 << i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i, 1 << i);
            }
        }
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

    pub fn apply_range(&mut self, mut l: usize, mut r: usize, f: F) {
        if l == r {
            return;
        }
        l += self.n;
        r += self.n;
        for i in (1..=self.log()).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i, 1 << i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i, 1 << i);
            }
        }
        {
            let mut l2 = l;
            let mut r2 = r;
            let mut len = 1;
            while l2 < r2 {
                if l2 & 1 == 1 {
                    self.all_apply(l2, &f, len);
                    l2 += 1;
                }
                if r2 & 1 == 1 {
                    r2 -= 1;
                    self.all_apply(r2, &f, len);
                }
                l2 >>= 1;
                r2 >>= 1;
                len <<= 1;
            }
        }
        for i in 1..=self.log() {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    pub fn all_prod(&self) -> S {
        self.data[1].clone()
    }

    fn log(&self) -> usize {
        64 - (self.n.leading_zeros() as usize) - 1
    }
}

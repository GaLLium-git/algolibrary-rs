fn main() {
    let op = |a: &i64, b: &i64| a + b;
    let apply = |a: &i64, f: &i64| a + f;
    let compose = |f: &i64, g: &i64| f + g;

    let id = 0i64;    // 単位元（和の単位元）
    let id_u = 0i64;  // 遅延操作の単位元（加算なので0）

    let mut seg = LazySegmentTree::new(8, op, apply, compose, id, id_u);

    for i in 0..8 {
        seg.set(i, i as i64); // seg = [0,1,2,3,4,5,6,7]
    }

    println!("get(3) = {}", seg.get(3));       // 3
    println!("prod(2,6) = {}", seg.prod(2, 6)); // 2+3+4+5=14

    seg.apply(2, 6, 10);  // 区間[2,6)に+10 → [0,1,12,13,14,15,6,7]

    println!("after apply(2,6,+10), get(3) = {}", seg.get(3));       // 13
    println!("prod(2,6) = {}", seg.prod(2, 6));                      // 12+13+14+15=54
}

pub struct LazySegmentTree<T, U, F, G, H>
where
    F: Fn(&T, &T) -> T,
    G: Fn(&T, &U) -> T,
    H: Fn(&U, &U) -> U,
    T: Clone,
    U: Clone + PartialEq,
{
    n: usize,
    size: usize,
    data: Vec<T>,
    lazy: Vec<U>,
    op: F,
    apply: G,
    compose: H,
    id: T,
    id_u: U,
}

impl<T, U, F, G, H> LazySegmentTree<T, U, F, G, H>
where
    F: Fn(&T, &T) -> T,
    G: Fn(&T, &U) -> T,
    H: Fn(&U, &U) -> U,
    T: Clone,
    U: Clone + PartialEq,
{
    pub fn new(size: usize, op: F, apply: G, compose: H, id: T, id_u: U) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        Self {
            n,
            size,
            data: vec![id.clone(); 2 * n],
            lazy: vec![id_u.clone(); 2 * n],
            op,
            apply,
            compose,
            id,
            id_u,
        }
    }

    fn height(&self) -> usize {
        self.n.trailing_zeros() as usize
    }

    fn update(&mut self, k: usize) {
        self.data[k] = (self.op)(&self.data[k << 1], &self.data[k << 1 | 1]);
    }

    fn all_apply(&mut self, k: usize, f: U) {
        self.data[k] = (self.apply)(&self.data[k], &f);
        if k < self.n {
            self.lazy[k] = (self.compose)(&self.lazy[k], &f);
        }
    }

    fn push(&mut self, k: usize) {
        if self.lazy[k] != self.id_u {
            self.all_apply(k << 1, self.lazy[k].clone());
            self.all_apply(k << 1 | 1, self.lazy[k].clone());
            self.lazy[k] = self.id_u.clone();
        }
    }

    pub fn set(&mut self, mut p: usize, value: T) {
        p += self.n;
        for i in (1..=self.height()).rev() {
            self.push(p >> i);
        }
        self.data[p] = value;
        for i in 1..=self.height() {
            self.update(p >> i);
        }
    }

    pub fn get(&mut self, mut p: usize) -> T {
        p += self.n;
        for i in (1..=self.height()).rev() {
            self.push(p >> i);
        }
        self.data[p].clone()
    }

    pub fn prod(&mut self, mut l: usize, mut r: usize) -> T {
        if l == r {
            return self.id.clone();
        }
        l += self.n;
        r += self.n;
        for i in (1..=self.height()).rev() {
            self.push(l >> i);
            self.push((r - 1) >> i);
        }
        let mut sml = self.id.clone();
        let mut smr = self.id.clone();
        while l < r {
            if (l & 1) == 1 {
                sml = (self.op)(&sml, &self.data[l]);
                l += 1;
            }
            if (r & 1) == 1 {
                r -= 1;
                smr = (self.op)(&self.data[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&sml, &smr)
    }

    pub fn apply(&mut self, mut l: usize, mut r: usize, f: U) {
        if l == r {
            return;
        }
        l += self.n;
        r += self.n;

        for i in (1..=self.height()).rev() {
            self.push(l >> i);
            self.push((r - 1) >> i);
        }

        let l0 = l;
        let r0 = r;

        while l < r {
            if (l & 1) == 1 {
                self.all_apply(l, f.clone());
                l += 1;
            }
            if (r & 1) == 1 {
                r -= 1;
                self.all_apply(r, f.clone());
            }
            l >>= 1;
            r >>= 1;
        }

        for i in 1..=self.height() {
            self.update(l0 >> i);
            self.update((r0 - 1) >> i);
        }
    }

    pub fn all_prod(&self) -> T {
        self.data[1].clone()
    }
}

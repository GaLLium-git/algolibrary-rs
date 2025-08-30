fn main() {
    let op = |a: &i64, b: &i64| a + b;
    let id = 0i64;
    let mut seg = SegmentTree::new(8, op, id);

    for i in 0..8 {
        seg.set(i, i as i64); // seg = [0, 1, 2, 3, 4, 5, 6, 7]
    }

    let val = seg.get(3);
    println!("get(3) = {}", val); // 3

    let r = seg.max_right(2, |x| *x <= 10);
    println!("max_right(2, <=10) = {}", r); // 5

    let l = seg.min_left(5, |x| *x < 8);
    println!("min_left(5, <8) = {}", l); // 3

    let q = seg.prod(2, 6);
    println!("prod(2,6) = {}", q); // 14

    let total = seg.all_prod();
    println!("all_prod() = {}", total); // 28
}

pub struct SegmentTree<T, F>
where
    F: Fn(&T, &T) -> T,
    T: Clone,
{
    n: usize,
    size: usize,
    data: Vec<T>,
    op: F,
    id: T,
}

impl<T, F> SegmentTree<T, F>
where
    F: Fn(&T, &T) -> T,
    T: Clone,
{
    pub fn new(size: usize, op: F, id: T) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        SegmentTree {
            n,
            size,
            data: vec![id.clone(); 2 * n],
            op,
            id,
        }
    }

    pub fn set(&mut self, mut i: usize, value: T) {
        i += self.n;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            self.data[i] = (self.op)(&self.data[i << 1], &self.data[i << 1 | 1]);
        }
    }

    pub fn get(&self, i: usize) -> T {
        self.data[i + self.n].clone()
    }

    pub fn prod(&self, mut l: usize, mut r: usize) -> T {
        l += self.n;
        r += self.n;
        let mut sml = self.id.clone();
        let mut smr = self.id.clone();
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

    pub fn all_prod(&self) -> T {
        self.data[1].clone()
    }

    pub fn max_right<G>(&self, mut l: usize, f: G) -> usize
    where
        G: Fn(&T) -> bool,
    {
        if l == self.size {
            return self.size;
        }
        l += self.n;
        let mut sm = self.id.clone();
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            let next = (self.op)(&sm, &self.data[l]);
            if !f(&next) {
                while l < self.n {
                    l <<= 1;
                    let t = (self.op)(&sm, &self.data[l]);
                    if f(&t) {
                        sm = t;
                        l += 1;
                    }
                }
                return l - self.n;
            }
            sm = next;
            l += 1;
            if (l & (l - 1)) == 0 {
                break;
            }
        }
        self.size
    }

    pub fn min_left<G>(&self, r: usize, f: G) -> usize
    where
        G: Fn(&T) -> bool,
    {
        if r == 0 {
            return 0;
        }
        let mut r = r + self.n;
        let mut sm = self.id.clone();
        loop {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            let next = (self.op)(&self.data[r], &sm);
            if !f(&next) {
                while r < self.n {
                    r = 2 * r + 1;
                    let t = (self.op)(&self.data[r], &sm);
                    if f(&t) {
                        sm = t;
                        r -= 1;
                    }
                }
                return r + 1 - self.n;
            }
            sm = next;
            if (r & (r - 1)) == 0 {
                break;
            }
        }
        0
    }
}





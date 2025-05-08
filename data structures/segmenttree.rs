fn main(){
  //hogehoge
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

    pub fn update(&mut self, mut i: usize, value: T) {
        i += self.n - 1;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            self.data[i] = (self.op)(&self.data[i << 1], &self.data[i << 1 | 1]);
        }
    }

    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.n - 1;
        r += self.n - 1;
        let mut res_left = self.id.clone();
        let mut res_right = self.id.clone();
        while l < r {
            if l & 1 == 1 {
                res_left = (self.op)(&res_left, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_right = (self.op)(&self.data[r], &res_right);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(&res_left, &res_right)
    }
}

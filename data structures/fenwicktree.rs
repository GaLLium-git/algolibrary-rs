fn main() {
    let mut ft = FenwickTree::new(10);
    ft.add(2, 5);
    ft.add(4, 3);
    ft.add(7, 2);

    println!("sum[0..5): {}", ft.range_sum(0, 5)); // 出力: 8
    println!("sum[2..8): {}", ft.range_sum(2, 8)); // 出力: 10
}



pub struct FenwickTree {
    size: usize,
    tree: Vec<i64>,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            size: n,
            tree: vec![0; n + 1],
        }
    }

    pub fn add(&mut self, mut i: usize, x: i64) {
        i += 1;
        while i <= self.size {
            self.tree[i] += x;
            i += i & (!i + 1);
        }
    }

    pub fn sum(&self, mut i: usize) -> i64 {
        let mut res = 0;
        while i > 0 {
            res += self.tree[i];
            i -= i & (!i + 1);
        }
        res
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - self.sum(l)
    }
}

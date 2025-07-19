fn main() {
    let v = vec![1, 3, 7, 9, 7, 4, 1];
    let max_pos = v.tsearch_max(|i, x| *x as i64);
    println!("最大値の位置: {}", max_pos); // 出力: 3

    let min_pos = v.tsearch_min(|i, x| *x as i64);
    println!("最小値の位置: {}", min_pos); // 出力: 0 または 6

    let peak = (1..=100).tsearch_range_max(|x| -(x - 50) * (x - 50));
    println!("最大値のx: {}", peak); // 出力: 50

    let valley = (1..=100).tsearch_range_min(|x| (x - 30) * (x - 30));
    println!("最小値のx: {}", valley); // 出力: 30
}

// ternary search
pub trait TernarySearch<T> {
    fn tsearch_max<F>(&self, f: F) -> usize
    where
        F: Fn(usize, &T) -> i64;

    fn tsearch_min<F>(&self, f: F) -> usize
    where
        F: Fn(usize, &T) -> i64;
}

impl<T> TernarySearch<T> for Vec<T> {
    fn tsearch_max<F>(&self, f: F) -> usize
    where
        F: Fn(usize, &T) -> i64,
    {
        let mut left = 0;
        let mut right = self.len() - 1;
        while right - left > 3 {
            let m1 = left + (right - left) / 3;
            let m2 = right - (right - left) / 3;
            if f(m1, &self[m1]) < f(m2, &self[m2]) {
                left = m1;
            } else {
                right = m2;
            }
        }

        (left..=right)
            .max_by_key(|&i| f(i, &self[i]))
            .unwrap()
    }

    fn tsearch_min<F>(&self, f: F) -> usize
    where
        F: Fn(usize, &T) -> i64,
    {
        let mut left = 0;
        let mut right = self.len() - 1;
        while right - left > 3 {
            let m1 = left + (right - left) / 3;
            let m2 = right - (right - left) / 3;
            if f(m1, &self[m1]) > f(m2, &self[m2]) {
                left = m1;
            } else {
                right = m2;
            }
        }

        (left..=right)
            .min_by_key(|&i| f(i, &self[i]))
            .unwrap()
    }
}

// ternary search for range
pub trait TernarySearchRange<T>
where
    T: Copy
        + PartialOrd
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + From<u8>
        + std::cmp::PartialEq,
{
    fn tsearch_range_max<F>(&self, f: F) -> T
    where
        F: Fn(T) -> i64;

    fn tsearch_range_min<F>(&self, f: F) -> T
    where
        F: Fn(T) -> i64;
}

impl<T, R> TernarySearchRange<T> for R
where
    R: std::ops::RangeBounds<T>,
    T: Copy
        + PartialOrd
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + From<u8>
        + std::cmp::PartialEq,
{
    fn tsearch_range_max<F>(&self, f: F) -> T
    where
        F: Fn(T) -> i64,
    {
        let mut l = match self.start_bound() {
            std::ops::Bound::Included(x) => *x,
            std::ops::Bound::Excluded(x) => *x + T::from(1),
            _ => panic!("Unbounded start not supported"),
        };
        let mut r = match self.end_bound() {
            std::ops::Bound::Included(x) => *x,
            std::ops::Bound::Excluded(x) => *x - T::from(1),
            _ => panic!("Unbounded end not supported"),
        };

        while (r - l) > T::from(3) {
            let m1 = l + (r - l) / T::from(3);
            let m2 = r - (r - l) / T::from(3);
            if f(m1) < f(m2) {
                l = m1;
            } else {
                r = m2;
            }
        }

        let mut best = l;
        let mut best_val = f(l);
        let mut cur = l + T::from(1);
        while cur <= r {
            let val = f(cur);
            if val > best_val {
                best = cur;
                best_val = val;
            }
            cur = cur + T::from(1);
        }

        best
    }

    fn tsearch_range_min<F>(&self, f: F) -> T
    where
        F: Fn(T) -> i64,
    {
        let mut l = match self.start_bound() {
            std::ops::Bound::Included(x) => *x,
            std::ops::Bound::Excluded(x) => *x + T::from(1),
            _ => panic!("Unbounded start not supported"),
        };
        let mut r = match self.end_bound() {
            std::ops::Bound::Included(x) => *x,
            std::ops::Bound::Excluded(x) => *x - T::from(1),
            _ => panic!("Unbounded end not supported"),
        };

        while (r - l) > T::from(3) {
            let m1 = l + (r - l) / T::from(3);
            let m2 = r - (r - l) / T::from(3);
            if f(m1) > f(m2) {
                l = m1;
            } else {
                r = m2;
            }
        }

        let mut best = l;
        let mut best_val = f(l);
        let mut cur = l + T::from(1);
        while cur <= r {
            let val = f(cur);
            if val < best_val {
                best = cur;
                best_val = val;
            }
            cur = cur + T::from(1);
        }

        best
    }
}

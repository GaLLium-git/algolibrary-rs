use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    pub coeffs: Vec<ModInt>,
}

impl Poly {
    pub fn new(mut coeffs: Vec<ModInt>) -> Self {
        while coeffs.last().map_or(false, |c| *c == ModInt::zero()) {
            coeffs.pop();
        }
        Self { coeffs }
    }

    fn resize(a: Vec<ModInt>, b: Vec<ModInt>) -> (Vec<ModInt>, Vec<ModInt>) {
        let n = a.len().max(b.len());
        let mut a = a;
        let mut b = b;
        a.resize(n, ModInt::zero());
        b.resize(n, ModInt::zero());
        (a, b)
    }
}

impl Add for Poly {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (a, b) = Self::resize(self.coeffs, rhs.coeffs);
        let coeffs = a.into_iter().zip(b).map(|(x, y)| x + y).collect();
        Self::new(coeffs)
    }
}

impl AddAssign for Poly {
    fn add_assign(&mut self, rhs: Self) {
        let (mut a, b) = Self::resize(self.coeffs.clone(), rhs.coeffs);
        for (i, bi) in b.into_iter().enumerate() {
            a[i] += bi;
        }
        *self = Self::new(a);
    }
}

impl Sub for Poly {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let (a, b) = Self::resize(self.coeffs, rhs.coeffs);
        let coeffs = a.into_iter().zip(b).map(|(x, y)| x - y).collect();
        Self::new(coeffs)
    }
}

impl SubAssign for Poly {
    fn sub_assign(&mut self, rhs: Self) {
        let (mut a, b) = Self::resize(self.coeffs.clone(), rhs.coeffs);
        for (i, bi) in b.into_iter().enumerate() {
            a[i] -= bi;
        }
        *self = Self::new(a);
    }
}

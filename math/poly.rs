fn main() {
    ModInt::set_modulus(998244353);

    let p = Poly::new(vec![ModInt::new(1),ModInt::new(2),ModInt::new(3)]); // 1 + 2x + 3x^2
    let q = Poly::new(vec![ModInt::new(4),ModInt::new(5)]);           // 4 + 5x

    println!("p*q = {:?}", (p.clone()*q.clone()).coeffs);
    println!("p shift 3 = {:?}",p.taylorshift(ModInt::new(3)).coeffs)
}


// use ModInt
// -------------------- Poly --------------------
//

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    pub coeffs: Vec<ModInt>,
}

impl Poly {
    pub fn new(mut coeffs: Vec<ModInt>) -> Self {
        Self { coeffs }
    }

    fn resize(mut a: Vec<ModInt>, mut b: Vec<ModInt>) -> (Vec<ModInt>, Vec<ModInt>) {
        let n = a.len().max(b.len());
        a.resize(n, ModInt::new(0));
        b.resize(n, ModInt::new(0));
        (a, b)
    }
    
    pub fn taylorshift(&self, c: ModInt) -> Self {
        let len=self.coeffs.len();
        let mut fact_mod = vec![ModInt::new(1);len];
        for i in 1..len{
            fact_mod[i]=fact_mod[i-1]*ModInt::new(i);
        }
        let mut v1 = self.coeffs.clone();
        for i in 0..len{
            v1[i]*=fact_mod[i];
        }
        v1.reverse();
        let mut v2 = vec![ModInt::new(1);len];
        for i in 1..len{
            v2[i]=v2[i-1]*c/ModInt::new(i);
        }
        let mut conv = convolution_mod(&v1,&v2);

        let mut res = vec![ModInt::new(0);len];
        for i in 0..len{
            res[i] = conv[i];
        }
        res.reverse();
        for i in 0..len{
            res[i] /= fact_mod[i];
        }
        Self::new(res)
    }
}

impl std::ops::Add for Poly {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (a, b) = Self::resize(self.coeffs, rhs.coeffs);
        let coeffs = a.into_iter().zip(b).map(|(x, y)| x + y).collect();
        Self::new(coeffs)
    }
}

impl std::ops::AddAssign for Poly {
    fn add_assign(&mut self, rhs: Self) {
        let (mut a, b) = Self::resize(self.coeffs.clone(), rhs.coeffs);
        for (i, bi) in b.into_iter().enumerate() {
            a[i] += bi;
        }
        *self = Self::new(a);
    }
}

impl std::ops::Sub for Poly {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let (a, b) = Self::resize(self.coeffs, rhs.coeffs);
        let coeffs = a.into_iter().zip(b).map(|(x, y)| x - y).collect();
        Self::new(coeffs)
    }
}

impl std::ops::SubAssign for Poly {
    fn sub_assign(&mut self, rhs: Self) {
        let (mut a, b) = Self::resize(self.coeffs.clone(), rhs.coeffs);
        for (i, bi) in b.into_iter().enumerate() {
            a[i] -= bi;
        }
        *self = Self::new(a);
    }
}

impl std::ops::Mul for Poly {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let coeffs = convolution_mod(&self.coeffs, &rhs.coeffs);
        Self::new(coeffs)
    }
}

impl std::ops::MulAssign for Poly {
    fn mul_assign(&mut self, rhs: Self) {
        let coeffs = convolution_mod(&self.coeffs, &rhs.coeffs);
        *self = Self::new(coeffs);
    }
}

//
// -------------------- NTT & Convolution --------------------
//

fn modpow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }
    result
}

fn modinv(x: u64, modulo: u64) -> u64 {
    modpow(x, modulo - 2, modulo)
}

fn bit_reverse(a: &mut [u64]) {
    let n = a.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
}

fn ntt(a: &mut [u64], modp: u64, root: u64) {
    let n = a.len();
    bit_reverse(a);
    let mut len = 2;
    while len <= n {
        let wlen = modpow(root, (modp - 1) / len as u64, modp);
        for i in (0..n).step_by(len) {
            let mut w = 1;
            for j in 0..len / 2 {
                let u = a[i + j];
                let v = a[i + j + len / 2] * w % modp;
                a[i + j] = (u + v) % modp;
                a[i + j + len / 2] = (modp + u - v) % modp;
                w = w * wlen % modp;
            }
        }
        len <<= 1;
    }
}

fn intt(a: &mut [u64], modp: u64, root: u64) {
    let n = a.len();
    bit_reverse(a);
    let mut len = 2;
    while len <= n {
        let wlen = modinv(modpow(root, (modp - 1) / len as u64, modp), modp);
        for i in (0..n).step_by(len) {
            let mut w = 1;
            for j in 0..len / 2 {
                let u = a[i + j];
                let v = a[i + j + len / 2] * w % modp;
                a[i + j] = (u + v) % modp;
                a[i + j + len / 2] = (modp + u - v) % modp;
                w = w * wlen % modp;
            }
        }
        len <<= 1;
    }
    let n_inv = modinv(n as u64, modp);
    for x in a.iter_mut() {
        *x = *x * n_inv % modp;
    }
}

fn convolution_mod_core(a: &[u64], b: &[u64], modp: u64, root: u64) -> Vec<u64> {
    let mut n = 1;
    while n < a.len() + b.len() - 1 {
        n <<= 1;
    }

    let mut fa = vec![0u64; n];
    let mut fb = vec![0u64; n];
    for i in 0..a.len() {
        fa[i] = a[i] % modp;
    }
    for i in 0..b.len() {
        fb[i] = b[i] % modp;
    }

    ntt(&mut fa, modp, root);
    ntt(&mut fb, modp, root);

    for i in 0..n {
        fa[i] = fa[i] * fb[i] % modp;
    }

    intt(&mut fa, modp, root);
    fa.resize(a.len() + b.len() - 1, 0);
    fa
}

fn garner(c1: &[u64], c2: &[u64], c3: &[u64], m1: u64, m2: u64, m3: u64, mod_final: u64) -> Vec<u64> {
    let m1_inv_m2 = modinv(m1 % m2, m2);
    let m12 = m1 as u128 * m2 as u128;
    let m12_inv_m3 = modinv((m1 * m2) % m3, m3);

    let mut result = Vec::with_capacity(c1.len());
    for i in 0..c1.len() {
        let x1 = c1[i];
        let x2 = ((c2[i] + m2 - x1 % m2) * m1_inv_m2) % m2;
        let x3 = ((c3[i] + m3 - (x1 + m1 * x2) % m3) * m12_inv_m3) % m3;
        let x = (x1 as u128 + (m1 as u128) * (x2 as u128) + m12 * (x3 as u128)) % ((m1 as u128) * (m2 as u128) * (m3 as u128));
        result.push((x % (mod_final as u128)) as u64);
    }
    result
}

pub fn convolution_mod(a: &[ModInt], b: &[ModInt]) -> Vec<ModInt> {
    const MOD1: u64 = 167772161;
    const MOD2: u64 = 469762049;
    const MOD3: u64 = 1224736769;
    const ROOT1: u64 = 3;
    const ROOT2: u64 = 3;
    const ROOT3: u64 = 3;

    let m = ModInt::modulus() as u64;

    let a_u64: Vec<u64> = a.iter().map(|x| x.val as u64).collect();
    let b_u64: Vec<u64> = b.iter().map(|x| x.val as u64).collect();

    let c1 = convolution_mod_core(&a_u64, &b_u64, MOD1, ROOT1);
    let c2 = convolution_mod_core(&a_u64, &b_u64, MOD2, ROOT2);
    let c3 = convolution_mod_core(&a_u64, &b_u64, MOD3, ROOT3);

    let res = garner(&c1, &c2, &c3, MOD1, MOD2, MOD3, m);
    res.into_iter().map(ModInt::new).collect()
}

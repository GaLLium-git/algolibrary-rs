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




// -------------------- Utilities --------------------

fn modpow(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut res = 1;
    a %= m;
    while b > 0 {
        if b % 2 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        b /= 2;
    }
    res
}

fn modinv(a: usize, m: usize) -> usize {
    modpow(a, m - 2, m)
}


// -------------------- Ntt Cache --------------------

pub struct NttCache {
    pub sum_e: Vec<usize>,
    pub sum_ie: Vec<usize>,
}

impl NttCache {
    pub fn new(n: usize, root: usize, modp: usize) -> Self {
        let cnt2 = (modp - 1).trailing_zeros() as usize;
        let mut es = vec![0; cnt2];
        let mut ies = vec![0; cnt2];

        let mut e = modpow(root, (modp - 1) >> cnt2, modp);
        let mut ie = modinv(e, modp);
        for i in (2..=cnt2).rev() {
            es[i - 2] = e;
            ies[i - 2] = ie;
            e = e * e % modp;
            ie = ie * ie % modp;
        }

        let mut sum_e = vec![0; 30];
        let mut acc = 1;
        for (i, &v) in es.iter().enumerate() {
            acc = acc * v % modp;
            sum_e[i] = acc;
        }

        let mut sum_ie = vec![0; 30];
        let mut acc = 1;
        for (i, &v) in ies.iter().enumerate() {
            acc = acc * v % modp;
            sum_ie[i] = acc;
        }
       

        NttCache { sum_e, sum_ie }
    }
}

// -------------------- NTT DIT --------------------

fn ntt_dit(a: &mut [usize], modp: usize, cache: &NttCache) {
    let n = a.len();
    let h = n.trailing_zeros() as usize;

    for ph in 1..=h {
        let w = 1 << (ph - 1);
        let p = 1 << (h - ph);
        let mut now = 1;
        for s in 0..w {
            let offset = s << (h - ph + 1);
            for i in 0..p {
                let l = a[i + offset];
                let r = a[i + offset + p] * now % modp;
                a[i + offset] = (l + r) % modp;
                a[i + offset + p] = (modp + l - r) % modp;
            }
            now = now * cache.sum_e[(!s as u32).trailing_zeros() as usize] % modp;
        }
    }
}

fn intt_dit(a: &mut [usize], modp: usize, cache: &NttCache) {
    let n = a.len();
    let h = n.trailing_zeros() as usize;

    for ph in (1..=h).rev() {
        let w = 1 << (ph - 1);
        let p = 1 << (h - ph);
        let mut inow = 1;
        for s in 0..w {
            let offset = s << (h - ph + 1);
            for i in 0..p {
                let l = a[i + offset];
                let r = a[i + offset + p];
                a[i + offset] = (l + r) % modp;
                a[i + offset + p] = (modp + l - r) * inow % modp;
            }
            inow = inow *  cache.sum_ie[(!s as u32).trailing_zeros() as usize] % modp;
        }
    }

    let n_inv = modinv(n, modp);
    for x in a.iter_mut() {
        *x = *x * n_inv % modp;
    }
}

// -------------------- Convolution --------------------

pub fn convolution_mod(a: &[ModInt], b: &[ModInt]) -> Vec<ModInt> {
    const MOD: usize = 998_244_353;
    const ROOT: usize = 3;

    let mut n = 1;
    while n < a.len() + b.len() - 1 {
        n <<= 1;
    }

    let cache = NttCache::new(n, ROOT, MOD);

    let mut fa = vec![0; n];
    let mut fb = vec![0; n];

    for i in 0..a.len() {
        fa[i] = a[i].val;
    }
    for i in 0..b.len() {
        fb[i] = b[i].val;
    }

    ntt_dit(&mut fa, MOD, &cache);
    ntt_dit(&mut fb, MOD, &cache);
    

    for i in 0..n {
        fa[i] = fa[i] * fb[i] % MOD;
    }
   

    intt_dit(&mut fa, MOD, &cache);
  
    fa.resize(a.len() + b.len() - 1, 0);
    fa.into_iter().map(ModInt::new).collect()
}

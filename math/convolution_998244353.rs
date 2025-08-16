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

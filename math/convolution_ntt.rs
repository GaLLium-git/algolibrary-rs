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

pub fn ntt(a: &mut [u64], modp: u64, root: u64) {
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

pub fn intt(a: &mut [u64], modp: u64, root: u64) {
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

pub fn convolution_mod_core(a: &[u64], b: &[u64], modp: u64, root: u64) -> Vec<u64> {
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
    const MOD1: u64 = 167772161;   // 2^25 * 5 + 1
    const ROOT1: u64 = 3;
    const MOD2: u64 = 469762049;   // 2^26 * 7 + 1
    const ROOT2: u64 = 3;
    const MOD3: u64 = 1224736769;  // 2^24 * 73 + 1
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

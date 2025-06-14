fn main() {
    let a = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7, 8];
    let c = convolution_and(&a, &b);
    println!("{:?}", c);
}


pub fn zeta_transform_and<T>(f: &mut [T])
where
    T: Copy + std::ops::AddAssign,
{
    let n = f.len().trailing_zeros() as usize;
    for i in 0..n {
        for mask in 0..f.len() {
            if (mask & (1 << i)) == 0 {
                f[mask] += f[mask | (1 << i)];
            }
        }
    }
}

pub fn mobius_transform_and<T>(f: &mut [T])
where
    T: Copy + std::ops::SubAssign,
{
    let n = f.len().trailing_zeros() as usize;
    for i in 0..n {
        for mask in 0..f.len() {
            if (mask & (1 << i)) == 0 {
                f[mask] -= f[mask | (1 << i)];
            }
        }
    }
}

pub fn convolution_and<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Copy
        + Default
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    let mut fa = a.to_vec();
    let mut fb = b.to_vec();
    zeta_transform_and(&mut fa);
    zeta_transform_and(&mut fb);
    for i in 0..fa.len() {
        fa[i] = fa[i] * fb[i];
    }
    mobius_transform_and(&mut fa);
    fa
}


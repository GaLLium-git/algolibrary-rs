fn main(){
    \\nothing here..
}

pub fn digits(n:usize,base:usize,limit:usize)->Vec<usize>{
    let mut v=Vec::new();
    let mut div=1;
    for _ in 0..limit{
        v.push((n/div)%base);
        div*=base;
    }
    v.reverse();
    v
}
 
pub fn gcd(n:usize,m:usize)->usize{
    if m==0 {
        n
    } else{
        gcd(m,n%m)
    }
}

pub fn is_prime(n:usize)->bool{
    let mut is_prime=true;
    for i in 2..=num::integer::sqrt(n){  //can use usize::isqrt in std from Rust 1.84
        if n%i==0{
            is_prime=false;
        }
    }
    is_prime
}

pub fn divisors(n:usize)->Vec<usize>{
    let mut divisors=vec![];
    for i in (1..=num::integer::sqrt(n)){
        if n%i==0{
            divisors.push(i);
            if !(i*i==n){
                divisors.push(n/i);
            }
        }
    }
    divisors
}

pub fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut d = 2;
    while d * d <= n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}





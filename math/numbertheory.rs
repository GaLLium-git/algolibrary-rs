pub mod numbertheory{
  pub fn digits(mut n:usize,base:usize)->Vec<usize>{
    let mut v=Vec::new();
    while n>0{
        v.push(n%base);
        n/=base;
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
}

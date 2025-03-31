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
  
}

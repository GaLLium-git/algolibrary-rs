fn main() {
   init!();
   let n=read!(usize);
   let mut a=read!(usize;n);
   let mut b=compress(&a);
   println!("{}",b.iter().join("\n"));
}
pub fn compress<T:Ord + Copy> (a:&Vec<T>)->Vec<usize>{
    let mut t=Vec::new();
    for i in 0..a.len(){
        t.push(a[i]);
    }
    t.sort();
    t.dedup();
    let mut b=Vec::new();
    for i in 0..a.len(){
        b.push(t.bsearch(|x| *x>=a[i]));
    }
    b
}

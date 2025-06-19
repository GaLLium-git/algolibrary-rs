fn main() {
   init!();
   let n=read!(usize);
   let mut a=read!(usize;n);
   let inf=usize::MAX;
   let mut lis=vec![inf;n];
   for &i in a.iter(){
     let pos=lis.bsearch(|x| *x>=i);
     lis[pos]=i;
   }
   let mut ans=0;
   for i in 0..n{
     if lis[i]==inf{
       break;
     }
     ans=i+1;
   }
   println!("{}",ans);
}

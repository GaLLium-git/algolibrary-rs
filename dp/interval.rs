fn main() {
   init!();
   let n=read!(usize);
   let mut p=vec![0;n+1];
   let mut a=vec![0;n+1];
   for i in 1..=n{
     (p[i],a[i])=read!(usize,usize);
   }
   
   let mut dp=vec![vec![0usize;n+1];n+1];
   for len in (0..n-1).rev(){
     for l in 1..=n-len{
       let r=l+len;
       if l==1{
         dp[l][r]=dp[l][r+1]+(if l<=p[r+1]&&p[r+1]<=r {a[r+1]} else {0});
       } else if r==n{
         dp[l][r]=dp[l-1][r]+(if l<=p[l-1]&&p[r-1]<=r {a[l-1]} else {0});
       } else {
         dp[l][r]=(dp[l][r+1]+(if l<=p[r+1]&&p[r+1]<=r {a[r+1]} else {0})).max(dp[l-1][r]+(if l<=p[l-1]&&p[l-1]<=r {a[l-1]} else {0}));
       }
     }
   }
   
   let mut ans=0;
   for i in 1..=n{
     ans=ans.max(dp[i][i]);
   }
   println!("{}",ans);
}


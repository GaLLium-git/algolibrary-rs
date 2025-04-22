fn main() {
    init!();
    let (n, weigh) = read!(usize, usize);
    let mut w=vec![0;n+1];
    let mut v=vec![0;n+1];
    for i in 1..=n{
        (w[i],v[i])=read!(usize,usize);
    }
    
    let mut dp=vec![vec![0usize;weigh+1];n+1];
   
    for i in 1..=n{
        for j in 0..=weigh{
            if j < w[i]{
                dp[i][j]=dp[i-1][j];
            } else {
                dp[i][j]=dp[i-1][j].max(dp[i-1][j-w[i]]+v[i]);
            }
        }
    }
    //println!("{:?}",dp[n]);
    println!("{}",dp[n].iter().max().unwrap());
}

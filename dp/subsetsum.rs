fn main() {
　　init!();
    let (n, s) = read!(usize, usize);
    let mut a = read!(usize;n);
    a.shift();
    let mut dp=vec![vec![false;s+1];n+1];
    dp[0][0]=true;
    for i in 1..=n{
        for j in 0..=s{
            if j < a[i]{
                dp[i][j]=dp[i-1][j];
            } else {
                dp[i][j]=dp[i-1][j] || dp[i-1][j-a[i]];
            }
        }
    }
    
    println!("{}",if dp[n][s] {"Yes"} else {"No"});
}

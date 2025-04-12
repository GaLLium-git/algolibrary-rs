fn main() {
    init!();
    
    let mut s :Vec<char> =read!(String).chars().collect();
    let mut t : Vec<char> =read!(String).chars().collect();
    s.shift();
    t.shift();
    let mut dp=vec![vec![0usize;t.len()];s.len()];
    for i in 1..s.len(){
        for j in 1..t.len(){
            if s[i]==t[j]{
                dp[i][j]=dp[i-1][j].max(dp[i][j-1]).max(dp[i-1][j-1]+1);
            } else {
                dp[i][j]=dp[i-1][j].max(dp[i][j-1]);
            }
        }
    }
   println!("{}",dp[s.len()-1][t.len()-1]);
}

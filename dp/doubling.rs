fn main(){
    init!();
    let (n,q)=read!(usize,usize);
    let mut dp=vec![vec![0;n+1];30];
    dp[0]=read!(usize;n);dp[0].shift();
    for i in 1..30{
        for j in 1..=n{
            dp[i][j]=dp[i-1][dp[i-1][j]];
        }
    }
    
    
    for _ in 0..q{
        let (x,y)=read!(usize,usize);
        let mut place=x;
        for i in 0..30{
           let bit=(y/(1<<i))%2;
           if bit==1{
               place=dp[i][place];
           }
        }
        println!("{}",place);
    }
    
}

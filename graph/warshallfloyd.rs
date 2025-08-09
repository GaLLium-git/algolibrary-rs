fn main() {
    init!();
    let (N,M) = read!(usize, usize);
    let mut d=vec![vec![usize::MAX/2;N+3];N+3];
    for i in 1..=N{
        d[i][i]=0;
    }
    for _ in 0..M{
        let (a,b,t)=read!(usize,usize,usize);
        d[a][b]=t;
        d[b][a]=t;
    }
    for k in 1..=N{
        for i in 1..=N{
            for j in 1..=N{
                d[i][j]=d[i][j].min(d[i][k]+d[k][j]);
            }
        }
    }
    let mut ans=usize::MAX;
    for i in 1..=N{
        let mut now_max=0;
        for j in 1..=N{
            now_max=now_max.max(d[i][j]);
        }
        ans=ans.min(now_max);
    }
    println!("{}",ans);
}

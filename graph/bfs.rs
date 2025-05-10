fn main() {
    init!();
    let (n,m)=read!(usize,usize);
    let mut graph=vec![vec![];n+1];
    for i in 1..=m{
      let (a,b)=read!(usize,usize);
      graph[a].push(b);
      graph[b].push(a);
    }
    
    let mut que=VecDeque::new();
    let mut dist=vec![-1i64;n+1];
    que.push_back(1);
    dist[1]=0;
    while !(que.is_empty()) {
        let pos=que.pop_front().unwrap();
        for &nex in graph[pos].iter(){
            if dist[nex]==-1 {
                dist[nex]=dist[pos]+1;
                que.push_back(nex);
            }
        }
    }
    
    for i in 1..=n{
        println!("{}",dist[i]);
    }
    
}
    

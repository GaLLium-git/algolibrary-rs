fn main() {
    init!();
    let (n,m)=read!(usize,usize);
    let mut graph=vec![vec![];n+1];
    for i in 1..=m{
      let (a,b,c)=read!(usize,usize,usize);
      graph[a].push((b,c));
      graph[b].push((a,c));
    }
    
    let mut kakutei=vec![false;n+1];
    let mut cur=vec![2000000000;n+1];
    let mut pq=BinaryHeap::new();
    cur[1]=0;
    pq.push(Reverse((cur[1],1)));
    
    while !(pq.is_empty()) {
        let Reverse(mintuple)=pq.pop().unwrap();
        let pos=mintuple.1;
        if kakutei[pos]==true{
            continue;
        }
        
        kakutei[pos]=true;
        for &nc in graph[pos].iter(){
            let (nex,cost)=nc;
            cur[nex]=cur[nex].min(cur[pos]+cost);
            pq.push(Reverse((cur[nex],nex)));
            
        }
    }
    
    for i in 1..=n{
        if cur[i]==2000000000{
            println!("{}",-1);
        } else{
            println!("{}",cur[i]);
        }
    }
    
}
    
    

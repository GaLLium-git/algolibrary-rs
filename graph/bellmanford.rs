fn main() {
    let mut sc=Scanner::new();
    let (n,m):(usize,usize)=(sc.next(),sc.next());
    let mut edges=vec![];
    for i in 0..m{
        let (a,b,c):(usize,usize,i64)=(sc.next(),sc.next(),sc.next());
        edges.push((a,b,-c));
    }
    let mut d=vec![i64::MAX;n+1];
    d[1]=0;
    for _ in 0..n-1{
        for &(u,v,cost) in edges.iter(){
            if d[u] != i64::MAX && d[v] > d[u]+cost{
                d[v]=d[u]+cost;
            }
        }
    }
    for _ in 0..n-1{
        for &(u,v,cost) in edges.iter(){
            if d[u] == i64::MIN || (d[u] != i64::MAX && d[v] > d[u]+cost){
                d[v]=i64::MIN;
            }
        }
    }
    
    
    if d[n]==i64::MIN{
        println!("inf");
    }else{
        println!("{}",-d[n]);
    }
    
    
}

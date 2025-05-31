// unionfindに依存  
fn main(){
    init!();
    let (n,m)=read!(usize,usize);
    let (mut a,mut b,mut c)=(vec![],vec![],vec![]);
    for i in 0..m{
        let (ap,bp,cp)=read!(usize,usize,usize);
        a.push(ap);
        b.push(bp);
        c.push((cp,i));
    }
    c.sort();
    
    let mut uf=UnionFind::new(n+1);
    let mut ans=0;
    for &(len,idx) in c.iter(){
        if !(uf.same(a[idx],b[idx])) {
            ans+=len;
            uf.union(a[idx],b[idx]);
        }
    }
    println!("{}",ans);
    
}
    

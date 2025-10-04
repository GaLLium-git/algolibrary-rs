// unionfindに依存  
fn main(){
    let mut sc=Scanner::new();
    let (n,m):(usize,usize)=(sc.next(),sc.next());
    let (mut a,mut b,mut c)=(vec![],vec![],vec![]);
    for i in 0..m{
        let (ap,bp,cp):(usize,usize,usize)=(sc.next(),sc.next(),sc.next());
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
    

fn main() {
    let (n, k) :(usize,usize)= (sc.next(),sc.next());
    let mut a :Vec<usize> =(0..n).map(|_| sc.next()).collect();
    a.shift();
    let mut shaku=vec![0usize;n+1];
    shaku[0]=1;
    for i in 1..=n{
        shaku[i]=shaku[i-1];
        while shaku[i]<n && a[shaku[i]+1]<=a[i]+k  {
            shaku[i]+=1;
        }
    }
    //println!("{:?}",shaku);
    let mut ans=0;
    for i in 1..=n{
        ans+=shaku[i]-i;
    }
    println!("{}", ans); 
}

fn dfs(v:usize,graph:&Vec<Vec<usize>>,seen:&mut Vec<bool>){
   seen[v]=true;
   for &nv in graph[v].iter(){
     if !(seen[nv]) {
       dfs(nv,graph,seen);
     }
   }
}
  
fn main() {
   init!();
   let (n,m)=read!(usize,usize);
   let mut graph=vec![vec![];n+1];
   for i in 1..=m{
     let (a,b)=read!(usize,usize);
     graph[a].push(b);
     graph[b].push(a);
   }
   
   let mut seen=vec![false;n+1];
   

   dfs(1,&graph,&mut seen);
   let mut ans="The graph is connected.";
   for i in 1..=n{
     if !(seen[i]) {
       ans="The graph is not connected."
     }
   }
   println!("{}",ans);
}

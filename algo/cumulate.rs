fn main(){
  let v=vec![1,2,3,3,4,5];
  let cumv=v.cumulate(std::ops::Add::add);
  println!("{:?}",cumv); //[1,3,6,9,13,18]
}
 //cumulate,cumlate_rev
pub trait Cumulate<T> 
    where
      T:Copy,
    {
    fn cumulate<F>(&self, f: F) -> Vec<T>
    where
        F: Fn(T,T) -> T;
     fn cumulate_rev<F>(&self, f: F) -> Vec<T>
     where
        F: Fn(T,T) -> T;
    }
impl<T> Cumulate<T> for Vec<T>
    where
      T:Copy,
    {
     fn cumulate<F>(&self, f: F) ->Vec<T>
     where
        F: Fn(T,T) ->T, 
      {
        let mut cumvec:Vec<T> = Vec::new();
        let mut accum=self[0];
        cumvec.push(accum);
        for i in 1..self.len() {
          accum=f(accum,self[i]);
          cumvec.push(accum);
        }
        cumvec
      }  
     fn cumulate_rev<F>(&self, f: F) ->Vec<T>
     where
        F: Fn(T,T) ->T, 
      {
        let mut cumvec:Vec<T> = Vec::new();
        let mut accum=self[self.len()-1];
        cumvec.push(accum);
        for i in (0..self.len()-1).rev() {
          accum=f(accum,self[i]);
          cumvec.push(accum);
        }
        cumvec.reverse();
        cumvec
      }  
    }
    
  //cumlate_2d
pub trait Cumulate2D<T> 
    where
      T:Copy,
    {
    fn cumulate_2d<F>(&self, f: F) -> Vec<Vec<T>>
    where
        F: Fn(T,T) -> T;
    }
impl<T> Cumulate2D<T> for Vec<Vec<T>>
    where
      T:Copy,
    {
     fn cumulate_2d<F>(&self, f: F) ->Vec<Vec<T>>
     where
        F: Fn(T,T) ->T, 
      {
        let mut cumvec:Vec<Vec<T>> = Vec::new();
        for i in 0..self.len(){
            cumvec.push(self[i].clone());
        }
        for i in 0..self.len() {
          for j in 1..self[i].len(){
            cumvec[i][j]=f(cumvec[i][j],cumvec[i][j-1]);
          }
        }
        for i in 1..self.len() {
          for j in 0..self[i].len(){
            cumvec[i][j]=f(cumvec[i][j],cumvec[i-1][j]);
          }
        }
        cumvec
      }  
    }

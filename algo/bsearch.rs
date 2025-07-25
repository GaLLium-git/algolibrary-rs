fn main(){
  let v=vec![1,2,3,3,4,5];
  let pos1=v.bsearch(|x| *x>=3);
  println!("{}",pos1); //2
  let pos2=bsearch_irange(1,100,|x| x>49);
  println!("{}",pos2); //50
}
 //bsearch
pub trait BinarySearch<T> {
    fn bsearch<F>(&self, f: F) -> usize
    where
        F: Fn(&T) -> bool;
  }
impl<T> BinarySearch<T> for Vec<T>{
     fn bsearch<F>(&self, f: F) -> usize
     where
        F: Fn(&T) -> bool, 
      {
        let mut left =0; 
        let mut right = self.len();
        while left!=right {
          let mid = left + (right - left) / 2;
          if f(&self[mid]) {right = mid;}
          else {left = mid+1;}
        }
        left
      }  
    }
   

pub fn bsearch_irange<F>(mut l: i64, mut r: i64, f: F) -> i64
where
    F: Fn(i64) -> bool,
{
    while l < r {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

pub fn bsearch_frange<F>(mut l: f64, mut r: f64, f: F, eps: f64) -> f64
where
    F: Fn(f64) -> bool,
{
    while r - l > eps {
        let m = (l + r) / 2.0;
        if f(m) {
            r = m;
        } else {
            l = m;
        }
    }
    l
}

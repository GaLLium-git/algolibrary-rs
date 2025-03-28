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
        let mut left = -1i64; 
        let mut right = self.len() as i64;
        while right - left > 1 {
          let mid = left + (right - left) / 2;
          if f(&self[mid as usize]) {right = mid;}
          else {left = mid;}
        }
        right as usize
      }  
    }
   
  //bsearch_range
pub trait BinarySearchRange<T>
    where
      T:From<u8>+PartialOrd
          +std::ops::Add<Output=T>
          +std::ops::Sub<Output=T>
          +std::ops::Mul<Output=T>
          +std::ops::Div<Output=T>,
    {
    fn bsearch_range<F>(&self, f: F) -> T
    where
        F: Fn(&T) -> bool;
    }

impl<T> BinarySearchRange<T> for std::ops::Range<T>
  where
      T:From<u8>+PartialOrd+Copy
          +std::ops::Add<Output=T>
          +std::ops::Sub<Output=T>
          +std::ops::Mul<Output=T>
          +std::ops::Div<Output=T>,
     {
      fn bsearch_range<F>(&self, f: F) -> T
      where
        F: Fn(&T) -> bool, 
      {
        let mut left = self.start-T::from(1);
        let mut right = self.end;
        while right - left > T::from(1){
          let mid = left + (right - left) / T::from(2);
          if f(&mid) {right = mid;}
          else {left = mid;}
        }
        right
      }
     }

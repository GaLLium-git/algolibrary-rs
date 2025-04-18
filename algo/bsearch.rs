fn main(){
  let v=vec![1,2,3,3,4,5];
  let pos1=v.bsearch(|x| *x>=3);
  println!("{}",pos1); //2
  let pos2=(1..=100).bsearch_range(|x| *x>49);
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
   
  //bsearch_range
pub trait BinarySearchRange<T>
    where
      T:From<u8>+PartialOrd+Copy
          +std::ops::Add<Output=T>
          +std::ops::Sub<Output=T>
          +std::ops::Mul<Output=T>
          +std::ops::Div<Output=T>,
    {
    fn bsearch_range<F>(&self, f: F) -> T
    where
        F: Fn(&T) -> bool;
    }

impl<S:std::ops::RangeBounds<T>,T> BinarySearchRange<T> for S
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
        let mut right = match self.end_bound() {
            std::ops::Bound::Included(right) => *right +T::from(1),
            std::ops::Bound::Excluded(right) => *right,
            std::ops::Bound::Unbounded => panic!("No Bounded Range in Binary Search"),
        };
        let mut left = match self.start_bound() {
            std::ops::Bound::Included(left) => *left,
            std::ops::Bound::Excluded(left) => *left + T::from(1),
            std::ops::Bound::Unbounded => panic!("No Bounded Range in Binary Search"),
        };

        assert!(left<=right);
        while left != right {
            let mid = left + (right - left) / T::from(2);
            if f(&mid) {right= mid;} 
            else { left= mid + T::from(1);}
        }
        left
      }
    }


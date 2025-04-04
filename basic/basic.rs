pub mod basic {
    //read! macro
    #[macro_export]
    macro_rules! init{
   (@inner $d:tt)=>{
      let input=std::io::read_to_string(std::io::stdin()).unwrap();
      let mut iter=input.split_whitespace();
      macro_rules! read{
       ($d t:ty) => {iter.next().unwrap().parse::<$d t>().unwrap()};
       ($d ($d t:ty),*) => {{ ($d (iter.next().unwrap().parse::<$d t>().unwrap(),)*)}};
       ($d t:ty; $d n:expr) => {(0..$d n).map(|_|read!($d t) ).collect::<Vec<_>>()};
       ($d ($d t:ty),*; $d n:expr) => {(0..$d n).map(|_|read!($dol ($d t),*)).collect::<Vec<_>>()};
       ($d t:ty; $d n:expr; $d m:expr) => {(0..$d m).map(|_|read!($d t; $d n)).collect::<Vec<_>>()};
       ($d ($d t:ty),*; $d n:expr; $d m:expr) => {(0..$d m).map(|_|read!($d ($d t),*; $d n)).collect::<Vec<_>>()};
       }
    };
    ()=>{init!(@inner $)};
   }

    //Shift vec
    pub trait Shift<T>
    where
        T: Default + Copy,
    {
        fn shift(&mut self);
        fn shifted(&mut self)->Vec<T>;
    }
    impl<T> Shift<T> for Vec<T>
    where
        T: Default + Copy,
    {
        fn shift(&mut self) {
            self.insert(0, T::default());
        }
        fn shifted(&mut self)->Vec<T>{
            self.shift();
            self.to_vec()
        }
    }
    
   
    pub trait Shift2D<T>
    where
        T: Default + Copy,
    {   
        fn shift(&mut self);
        fn shifted(&mut self)->Vec<Vec<T>>;
        fn shift_2d(&mut self);
        fn shifted_2d(&mut self)->Vec<Vec<T>>;
    }
    impl<T> Shift2D<T> for Vec<Vec<T>>
    where
        T: Default + Copy,
    {
         fn shift(&mut self) {
            self.insert(0, vec![T::default();self[0].len()]);
        }
        fn shifted(&mut self)-> Vec<Vec<T>>{
           self.shift();
           self.to_vec()
        }
        fn shift_2d(&mut self) {
            for i in 0..self.len() {
                self[i].shift();
            }
            self.shift();
        }
        fn shifted_2d(&mut self)->Vec<Vec<T>>{
            self.shift_2d();
            self.to_vec()
        }
    }

    //bsearch
    pub trait BinarySearch<T> {
        fn bsearch<F>(&self, f: F) -> usize
        where
            F: Fn(&T) -> bool;
    }
    impl<T> BinarySearch<T> for Vec<T> {
        fn bsearch<F>(&self, f: F) -> usize
        where
            F: Fn(&T) -> bool,
        {
            let mut left = 0;
            let mut right = self.len();
            while left != right {
                let mid = left + (right - left) / 2;
                if f(&self[mid]) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        }
    }

    //bsearch_range
    pub trait BinarySearchRange<T>
    where
        T: From<u8>
            + PartialOrd
            + Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>,
    {
        fn bsearch_range<F>(&self, f: F) -> T
        where
            F: Fn(&T) -> bool;
    }

    impl<S: std::ops::RangeBounds<T>, T> BinarySearchRange<T> for S
    where
        T: From<u8>
            + PartialOrd
            + Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>,
    {
        fn bsearch_range<F>(&self, f: F) -> T
        where
            F: Fn(&T) -> bool,
        {
            let mut right = match self.end_bound() {
                std::ops::Bound::Included(right) => *right + T::from(1),
                std::ops::Bound::Excluded(right) => *right,
                std::ops::Bound::Unbounded => panic!("No Bounded Range in Binary Search"),
            };
            let mut left = match self.start_bound() {
                std::ops::Bound::Included(left) => *left,
                std::ops::Bound::Excluded(left) => *left + T::from(1),
                std::ops::Bound::Unbounded => panic!("No Bounded Range in Binary Search"),
            };

            assert!(left <= right);
            while left != right {
                let mid = left + (right - left) / T::from(2);
                if f(&mid) {
                    right = mid;
                } else {
                    left = mid + T::from(1);
                }
            }
            left
        }
    }

    // cumulate,cumlate_rev
    pub trait Cumulate<T>
    where
        T: Copy,
    {
        fn cumulate<F>(&self, f: F) -> Vec<T>
        where
            F: Fn(T, T) -> T;
        fn cumulate_rev<F>(&self, f: F) -> Vec<T>
        where
            F: Fn(T, T) -> T;
    }
    impl<T> Cumulate<T> for Vec<T>
    where
        T: Copy,
    {
        fn cumulate<F>(&self, f: F) -> Vec<T>
        where
            F: Fn(T, T) -> T,
        {
            let mut cumvec: Vec<T> = Vec::new();
            let mut accum = self[0];
            cumvec.push(accum);
            for i in 1..self.len() {
                accum = f(accum, self[i]);
                cumvec.push(accum);
            }
            cumvec
        }
        fn cumulate_rev<F>(&self, f: F) -> Vec<T>
        where
            F: Fn(T, T) -> T,
        {
            let mut cumvec: Vec<T> = Vec::new();
            let mut accum = self[self.len() - 1];
            cumvec.push(accum);
            for i in (0..self.len() - 1).rev() {
                accum = f(accum, self[i]);
                cumvec.push(accum);
            }
            cumvec.reverse();
            cumvec
        }
    }

    //cumlate_2d
    pub trait Cumulate2D<T>
    where
        T: Copy,
    {
        fn cumulate_2d<F>(&self, f: F) -> Vec<Vec<T>>
        where
            F: Fn(T, T) -> T;
    }
    impl<T> Cumulate2D<T> for Vec<Vec<T>>
    where
        T: Copy,
    {
        fn cumulate_2d<F>(&self, f: F) -> Vec<Vec<T>>
        where
            F: Fn(T, T) -> T,
        {
            let mut cumvec: Vec<Vec<T>> = Vec::new();
            for i in 0..self.len() {
                cumvec.push(self[i].clone());
            }
            for i in 0..self.len() {
                for j in 1..self[i].len() {
                    cumvec[i][j] = f(cumvec[i][j], cumvec[i][j - 1]);
                }
            }
            for i in 1..self.len() {
                for j in 0..self[i].len() {
                    cumvec[i][j] = f(cumvec[i][j], cumvec[i - 1][j]);
                }
            }
            cumvec
        }
    }
}

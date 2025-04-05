fn main(){
    let mut v=vec![1,2,3,4,5];
    v.shift();
    println!("{:?}",v);
}

pub trait Shift<T>
    where
        T: Default + Copy,
    {
        fn shift(&mut self);
    }
    impl<T> Shift<T> for Vec<T>
    where
        T: Default + Copy,
    {
        fn shift(&mut self) {
            self.insert(0, T::default());
        }
    }
    
   
pub trait Shift2D<T>
    where
        T: Default + Copy,
    {   
        fn shift(&mut self);
        fn shift_2d(&mut self);
    }
    impl<T> Shift2D<T> for Vec<Vec<T>>
    where
        T: Default + Copy,
    {
         fn shift(&mut self) {
            self.insert(0, vec![T::default();self[0].len()]);
        }
        fn shift_2d(&mut self) {
            for i in 0..self.len() {
                self[i].shift();
            }
            self.shift();
        }
    }

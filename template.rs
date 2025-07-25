#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use library::*;
use itertools::*;
use ordered_float::*;
use std::collections::*;

macro_rules! init{
    (@inner $d:tt)=>{
        let input=std::io::read_to_string(std::io::stdin()).unwrap();
        let mut iter=input.split_whitespace();
        macro_rules! read{
            ($d t:ty) => {iter.next().unwrap().parse::<$d t>().unwrap()};
            ($d ($d t:ty),*) => {{ ($d (iter.next().unwrap().parse::<$d t>().unwrap(),)*)}};
            ($d t:ty; $d n:expr) => {(0..$d n).map(|_|read!($d t) ).collect::<Vec<_>>()};
            ($d ($d t:ty),*; $d n:expr) => {(0..$d n).map(|_|read!($d ($d t),*)).collect::<Vec<_>>()};
            ($d t:ty; $d n:expr; $d m:expr) => {(0..$d m).map(|_|read!($d t; $d n)).collect::<Vec<_>>()};
            ($d ($d t:ty),*; $d n:expr; $d m:expr) => {(0..$d m).map(|_|read!($d ($d t),*; $d n)).collect::<Vec<_>>()};
        }
    };
    ()=>{init!(@inner $)};
}

fn main() {
    init!();
    let (a,b) = read!(usize, usize);
    let ans = if a*b%2==0 {"Even"} else {"Odd"};
    println!("{}",ans);
}



pub mod library {

//Shift vec
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

 //cumulate,cumlate_rev
pub trait Cumulate<T> 
    where
      T:Clone,
    {
    fn cumulate<F>(&self, f: F) -> Vec<T>
    where
        F: Fn(&T,&T) -> T;
     fn cumulate_rev<F>(&self, f: F) -> Vec<T>
     where
        F: Fn(&T,&T) -> T;
    }
impl<T> Cumulate<T> for Vec<T>
    where
      T:Clone,
    {
     fn cumulate<F>(&self, f: F) ->Vec<T>
     where
        F: Fn(&T,&T) ->T, 
      {
        let mut cumvec= self.clone();
        for i in 1..self.len() {
          cumvec[i]=f(&cumvec[i-1],&self[i]);
        }
        cumvec
      }  
     fn cumulate_rev<F>(&self, f: F) ->Vec<T>
     where
        F: Fn(&T,&T) ->T, 
      {
        let mut cumvec = self.clone();
        for i in (0..self.len()-1).rev() {
          cumvec[i]=f(&cumvec[i+1],&self[i]);
        }
        cumvec
      }
    }
    
  //cumlate_2d
pub trait Cumulate2D<T> 
    where
      T:Clone,
    {
    fn cumulate_2d<F>(&self, f: F) -> Vec<Vec<T>>
    where
        F: Fn(&T,&T) -> T;
    }
impl<T> Cumulate2D<T> for Vec<Vec<T>>
    where
      T:Clone,
    {
     fn cumulate_2d<F>(&self, f: F) ->Vec<Vec<T>>
     where
        F: Fn(&T,&T) ->T, 
      {
        let mut cumvec = self.clone();
        for i in 0..self.len() {
          for j in 1..self[i].len(){
            cumvec[i][j]=f(&cumvec[i][j],&cumvec[i][j-1]);
          }
        }
        for i in 1..self.len() {
          for j in 0..self[i].len(){
            cumvec[i][j]=f(&cumvec[i][j],&cumvec[i-1][j]);
          }
        }
        cumvec
      }  
    }
    
}

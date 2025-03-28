#![allow(unused)]
#![allow(non_snake_case)]
#![allow(dead_code)]
use basic::*;
use std::collections::*;
use itertools::*;
use ordered_float::*;

fn main(){
  init!();
  let a=read!(i64);
  let b=read!(i64);
  println!("{}",if a*b%2==1 {"Odd"}else{"Even"});
}

pub mod basic{
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
   
  //to_1indexed
  pub trait To1Index<T>
    where
      T:Default+Copy,
    {
    fn to_1index(&mut self);
    }
  impl<T> To1Index<T> for Vec<T>
     where
        T:Default+Copy,
    {
     fn to_1index(&mut self){
         self.insert(0,T::default());
     }
    }
  impl<T> To1Index<T> for Vec<Vec<T>>
    where
      T:Default+Copy,
    {
    fn to_1index(&mut self){
       for i in 0..self.len(){
         self[i].insert(0,T::default());
       }
       self.insert(0,vec![T::default();self[1].len()]);
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
}


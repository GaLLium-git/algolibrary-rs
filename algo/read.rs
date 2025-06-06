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

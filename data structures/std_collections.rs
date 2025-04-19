//example of uses
//stack 実装はVecDeque
fn main() {
    init!();
    let q= read!(usize);
    let mut stack=VecDeque::new();
    for i in 0..q{
      let query=read!(usize);
      match query {
        1 =>{stack.push_back(read!(String))},  
        2 =>{println!("{}",*stack.back().unwrap());}, //backの返り値はOption<&T> (参照なのはmoveを防ぐため)
        3 =>{stack.pop_back();},   //pop_backの返り値はOption<T>
        _ =>{},
      }
    }
}
//queue 実装はVecDeque
fn main() {
    init!();
    let q= read!(usize);
    let mut queue=VecDeque::new();
    for i in 0..q{
      let query=read!(usize);
      match query {
        1 =>{queue.push_back(read!(String))},  //frontの返り値はOption<&T> (参照なのはmoveを防ぐため)
        2 =>{println!("{}",*queue.front().unwrap());},  //pop_frontの返り値はOption<T>
        3 =>{queue.pop_front();},
        _ =>{},
      }
    }
}

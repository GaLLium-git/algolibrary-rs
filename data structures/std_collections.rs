//example of uses
//大事なこと：&Tなのは大体所有権のmoveを防ぐため

//stack 実装はVecDeque
fn main() {
    init!();
    let q= read!(usize);
    let mut stack=VecDeque::new();
    for i in 0..q{
      let query=read!(usize);
      match query {
        1 =>{stack.push_back(read!(String))},  
        2 =>{println!("{}",*stack.back().unwrap());}, //backの返り値はOption<&T> 
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
        1 =>{queue.push_back(read!(String))},  //frontの返り値はOption<&T> 
        2 =>{println!("{}",*queue.front().unwrap());},  //pop_frontの返り値はOption<T>
        3 =>{queue.pop_front();},
        _ =>{},
      }
    }
}

//priority queue 実装はBinaryHeap
fn main() {
    init!();
    let q= read!(usize);
    let mut pq=BinaryHeap::new();
    for i in 0..q{
        let query=read!(usize);
        match query {
            1 =>{pq.push(read!(usize));},
            2 =>{println!("{}",*pq.peek().unwrap());}, //peekの返り値はOption<&T> 
            3 =>{pq.pop();},    //popの返り値はOption<T>
            _ =>{},
        }
    }
}

//map 実装はHashMap
fn main() {
    init!();
    let q= read!(usize);
    let mut map=HashMap::new();
    for i in 0..q{
        let query=read!(usize);
        match query {
            1 =>{map.insert(read!(String),read!(usize));},
            2 =>{println!("{}",*map.get(&read!(String)).unwrap());},  //getの引数は&T,返り値はOption<&T> 
            _ =>{},
        }
    }
}

//set 実装はBTreeSet
fn main() {
    init!();
    let q= read!(usize);
    let mut set=BTreeSet::new();
    for i in 0..q{
        let query=read!(usize);
        match query {
            1 =>{set.insert(read!(usize));},
            2 =>{set.remove(&read!(usize));},  //removeの引数は&T
            3 =>{if let Some(&ans)=set.range(read!(usize)..).next() {println!("{}",ans);} //rangeの返り値は&Tのiterator
                 else {println!("{}",-1);} },
            _ =>{},
        }
    }
}

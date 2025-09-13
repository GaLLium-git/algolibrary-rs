fn main(){
    let mut sc=Scanner::new();
    let n:usize=sc.next();
    println!("{}",n);
    let a:Vec<usize>=(0..n).map(|_| sc.next()).collect();
    println!("{:?}",a);
}

pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
}
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn read_line(&mut self) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        self.buffer = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                return token.parse::<T>().ok().unwrap();
            }
            self.read_line();
        }
    }
}



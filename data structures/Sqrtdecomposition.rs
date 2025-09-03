fn main() {
    init!();
    let (N,M)=read!(usize,usize);
    let size=N+5;
    let id =0usize;
    let op=|a:&usize, b:&usize| *a + *b;
    let mapping=|a:&usize,b:&usize| (*a).max(*b);
    let mut sq=SqrtDecomposition::new(size,id,op,mapping);
    let mut ans=0;
    for _ in 0..M{
        let (t,l,r)=read!(usize,usize,usize);
        let nts=(r-l+1)*t;
        let lts=sq.prod(l,r+1);
        ans+=(nts-lts);
        sq.apply(l,r+1,t);
    }
    println!("{}",ans);
}

#[derive(Clone)]
struct Block<T> {
    data: Vec<T>,
    acc: T,
    lazy: Option<T>,
}

pub struct SqrtDecomposition<T, F, G>
where
    T: Copy,
    F: Fn(&T, &T) -> T,
    G: Fn(&T, &T) -> T,
{
    n: usize,
    b: usize,
    id: T,
    op: F,
    mapping: G,
    blocks: Vec<Block<T>>,
}

impl<T, F, G> SqrtDecomposition<T, F, G>
where
    T: Copy,
    F: Fn(&T, &T) -> T,
    G: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, id: T, op: F, mapping: G) -> Self {
        let b = (n as f64).sqrt().ceil() as usize;
        let mut blocks = vec![];
        for i in (0..n).step_by(b) {
            let len = (n - i).min(b);
            blocks.push(Block {
                data: vec![id; len],
                acc: id,
                lazy: None,
            });
        }
        Self {
            n,
            b,
            id,
            op,
            mapping,
            blocks,
        }
    }

    fn push(&mut self, block_id: usize) {
        if let Some(val) = self.blocks[block_id].lazy.take() {
            let block = &mut self.blocks[block_id];
            for v in block.data.iter_mut() {
                *v = (self.mapping)(&val, v);
            }
            block.acc = block.data.iter().fold(self.id, |a, x| (self.op)(&a, x));
        }
    }

    pub fn set(&mut self, i: usize, x: T) {
        let block_id = i / self.b;
        self.push(block_id);
        let idx = i % self.b;
        self.blocks[block_id].data[idx] = x;
        self.blocks[block_id].acc = self.blocks[block_id]
            .data
            .iter()
            .fold(self.id, |a, x| (self.op)(&a, x));
    }

    pub fn get(&mut self, i: usize) -> T {
        let block_id = i / self.b;
        self.push(block_id);
        self.blocks[block_id].data[i % self.b]
    }

    pub fn prod(&mut self, l: usize, r: usize) -> T {
        let mut res = self.id;
        let mut i = l;
        while i < r {
            let block_id = i / self.b;
            let block_start = block_id * self.b;
            let block_end = (block_start + self.blocks[block_id].data.len()).min(r);

            if i == block_start && block_end <= r && block_end - block_start == self.blocks[block_id].data.len() {
                if let Some(val) = self.blocks[block_id].lazy {
                    let tmp = self.blocks[block_id].data.len();
                    res = (self.op)(&res, &(0..tmp).fold(self.id, |a, _| (self.op)(&a, &val)));
                } else {
                    res = (self.op)(&res, &self.blocks[block_id].acc);
                }
                i = block_end;
            } else {
                self.push(block_id);
                while i < block_end {
                    res = (self.op)(&res, &self.blocks[block_id].data[i % self.b]);
                    i += 1;
                }
            }
        }
        res
    }

    pub fn apply(&mut self, l: usize, r: usize, val: T) {
        let mut i = l;
        while i < r {
            let block_id = i / self.b;
            let block_start = block_id * self.b;
            let block_end = (block_start + self.blocks[block_id].data.len()).min(r);

            if i == block_start && block_end <= r && block_end - block_start == self.blocks[block_id].data.len() {
                if let Some(prev) = self.blocks[block_id].lazy {
                    self.blocks[block_id].lazy = Some((self.mapping)(&val, &prev));
                } else {
                    self.blocks[block_id].lazy = Some(val);
                }
                i = block_end;
            } else {
                self.push(block_id);
                while i < block_end {
                    let idx = i % self.b;
                    self.blocks[block_id].data[idx] =
                        (self.mapping)(&val, &self.blocks[block_id].data[idx]);
                    i += 1;
                }
                self.blocks[block_id].acc = self.blocks[block_id]
                    .data
                    .iter()
                    .fold(self.id, |a, x| (self.op)(&a, x));
            }
        }
    }
}

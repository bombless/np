#![feature(iterator_step_by)]

extern crate fixedbitset;



const MAX: usize  = 80000000;

struct Pool(fixedbitset::FixedBitSet);

impl Pool {
    fn new() -> Self {
        Pool(fixedbitset::FixedBitSet::with_capacity(MAX))
    }

    fn get(&self, n: usize) -> bool {
        self.0.contains(n)
    }

    fn set(&mut self, n: usize) {
        self.0.insert(n)
    }
}

fn main() {
    let mut pool = Pool::new();
    for i in (3 .. 10000).step_by(2) {
        let mut j = i;
        while i * j < MAX {
            pool.set(i * j);
            j += 2
        }
    }
    let mut count = 1;
    for i in (3 .. MAX).step_by(2) {
        if !pool.get(i) {
            count += 1
        }
        if count == 4263116 {
            println!("{}", i);
            return
        }
    }
}

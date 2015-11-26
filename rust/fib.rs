// compile: rustc -O fib.rs -o fib.rust
// run: time ./fib.rust 5000000

use std::*;
fn fib(i: u64) -> u64 {
    return fib2(0, 1, i)
}

fn fib2(i1: u64, i2: u64, i: u64) -> u64 {
   if i > 0 {
       return fib2(i2, i1+i2, i-1);
   } else {
       return i2;
   }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let n: u64 = args[1].parse::<u64>().unwrap();
    let result = fib(n);
    println!("LANGUAGE Rust: {0}", result);
}

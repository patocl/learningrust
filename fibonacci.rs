// to run it https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=63f26b1858a2e8cdc5ea6e2e8bef57de

const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

fn fib(n: u64) -> u64 {
    if n==0 {
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
      fib(n-1) + fib(n-2) 
    }
}

fn main() {
    let count = 41;
    println!("fibonnaci of first {} numbers", count - 1);
    for i in 1..count {
        println!("{} = {}", i, fib(i))
    }
}

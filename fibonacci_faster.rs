// to run it https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=63f26b1858a2e8cdc5ea6e2e8bef57de

use std::collections::HashMap;

fn fib_faster(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = fib_faster(n-1, map) + fib_faster(n-2, map);
                map.insert(n, val);
                val
            }
        }
    }
}


fn main() {
    let count = 41;
    println!("fibonnaci of first {} numbers", count);
    let mut map = HashMap::new();
    for i in 1..count {
        println!("{} = {}", i, fib_faster(i, &mut map))
    }
}

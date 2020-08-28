// To run it go to https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e0cf34d70e00c269ee1437fbb2b937e8

use std::env;

fn main() {
    let vars = env::vars();
    
    for (key, val) in vars {
        println!("{}:{}", key, val);
    }
}

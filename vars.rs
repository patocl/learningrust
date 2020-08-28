use std::env;

fn main() {
    let vars = env::vars();
    
    for (key, val) in vars {
        println!("{}:{}", key, val);
    }
}

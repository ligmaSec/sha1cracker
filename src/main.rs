use std::env;

fn main() {
    let args = env::args();


    println!("{:?}",args.last().unwrap());
}

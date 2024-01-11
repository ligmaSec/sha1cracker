use std::{env, process, str};
use std::fs::read_to_string;
use sha1_smol::Sha1;


const SHA1_HEX_STRING_LENGTH: usize = 40;

fn print_usage() {
       println!("{}",String::from("Usage: ./sha1cracker [WORDLIST-FILE] [SHA1-HASH]"));
}

fn read_lines(filename: &str)-> Vec<String> {
    read_to_string(filename)
        .unwrap() //panic on file read error
        .lines() // returns iterator
        .map(String::from) //convert each line to a String
        .collect() // satisfy vector return
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage();
        process::exit(1);
    }

    let wordlist = args[1].trim();
    let sha1_hash = args[2].trim();
    //if sha1_hash.len() != SHA1_HEX_STRING_LENGTH {
    //    println!("invalid hash length, must be 40 characters");
    //}
    
    //let mut word: &[u8] = "hello".as_bytes();

    let mut hasher = Sha1::new();
    //hasher.update(word);
    for line in read_lines(wordlist){
       println!("{:?}",line.as_bytes()); 
    }
    



    //assert_eq!(hasher.digest().to_string(), "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    //println!("the hash cleartext is {}", str::from_utf8(word).unwrap());

}

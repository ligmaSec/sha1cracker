use std::{env, process, str};
use std::fs::File;
use sha1_smol::Sha1;
use std::io::Read;


const SHA1_HEX_STRING_LENGTH: usize = 40;

fn print_usage() {
       println!("{}",String::from("Usage: ./sha1cracker [WORDLIST-FILE] [SHA1-HASH]"));
}

fn read_lines(mut filename: String)-> Vec<u8> {
    let mut file_content;
    let mut file = Read::read_to_end(&mut file_content).expect("unable to open file");

    file.read_to_end(file_content);
    file_content
    
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage();
        process::exit(1);
    }

    let wordlist = args[1].trim();
    let sha1_hash = args[2].trim();
    if sha1_hash.len() != SHA1_HEX_STRING_LENGTH {
        println!("invalid hash length, must be 40 characters");
    }
    
    //let mut word: &[u8] = "hello".as_bytes();

    let mut hasher = Sha1::new();
    //hasher.update(word);
    for line in read_lines(wordlist).lines(){
       hasher.update(line.as_bytes());
       println!("{}", hasher.digest().to_string());
       if hasher.digest().to_string() == sha1_hash.to_string() {
            println!("the clear text hash is {}",line);
            process::exit(0);
       }
       hasher.reset();
    }
    



    //assert_eq!(hasher.digest().to_string(), "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    //println!("the hash cleartext is {}", str::from_utf8(word).unwrap());

}

use std::error::Error;
use std::{env, process};
use std::fs::File;
use sha1_smol::Sha1;
use std::io::{BufRead, BufReader};


const SHA1_HEX_STRING_LENGTH: usize = 40;

fn print_usage() {
       println!("{}",String::from("Usage: ./sha1cracker [WORDLIST-FILE] [SHA1-HASH]"));
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage();
        return Ok(());
    }

    let wordlist = args[1].trim();
    let sha1_hash = args[2].trim();
    if sha1_hash.len() != SHA1_HEX_STRING_LENGTH {
        println!("invalid hash length, must be 40 characters");
        return Ok(());
    }
   
    let file = File::open(wordlist).expect("file not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let word = line.trim();

        if sha1_hash == Sha1::from(word).digest().to_string() {
            println!("the hash cleartext is {}", word);
            return Ok(());

        }
    
    }
    Ok(())

    //assert_eq!(hasher.digest().to_string(), "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
    //println!("the hash cleartext is {}", str::from_utf8(word).unwrap());

}

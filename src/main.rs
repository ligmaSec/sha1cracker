use std::error::Error;
use std::env;
use std::fs::File;
use sha1_smol::Sha1;
use std::io::{Read, BufReader};

// more hashing algorithms will be added in the future
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
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let file_contents = String::from_utf8_lossy(&buffer);

    for line in file_contents.lines() {
        let word = line.trim();

        if sha1_hash == Sha1::from(word).digest().to_string() {
            println!("the hash cleartext is {}", word);
            return Ok(());

        }
    
    }
    Ok(())


}

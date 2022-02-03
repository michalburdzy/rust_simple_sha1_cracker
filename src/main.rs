use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();

    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let word_list_file = File::open(&args[1])?;
    let reader = BufReader::new(&word_list_file);

    for line in reader.lines() {
        let line = line?.trim().to_string();
        let password_candidate = line.trim();
        let word_hash = &hex::encode(sha1::Sha1::digest(password_candidate.as_bytes()));
        if hash_to_crack == word_hash {
            println!("Password found: {}", &password_candidate);

            return Ok(());
        } else {
            println!("Password not found in a wordlist");

            return Ok(());
        }
    }

    Ok(())
}

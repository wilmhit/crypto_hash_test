use crypto::sha1::Sha1;
use crypto::sha2::{Sha256, Sha512};
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let plain_text = String::from(args[args.len()-1].as_str());
    println!("String to encode: {}", plain_text);
    //Sha1
    let mut start_time = Instant::now();
    let mut cipher_text = get_sha1(&plain_text);
    println!("Sha1 time: {}ns Encrypted: {}",
        start_time.elapsed().as_nanos(), cipher_text);
    //Sha2-256
    start_time = Instant::now();
    cipher_text = get_sha2_256(&plain_text);
    println!("Sha2-256 time: {}ns Encrypted: {}",
        start_time.elapsed().as_nanos(), cipher_text);
    //Sha2-512
    start_time = Instant::now();
    cipher_text = get_sha2_512(&plain_text);
    println!("Sha2-512 time: {}ns Encrypted: {}",
        start_time.elapsed().as_nanos(), cipher_text);
    //Sha3-256
    start_time = Instant::now();
    cipher_text = get_sha3_256(&plain_text);
    println!("Sha3-256 time: {}ns Encrypted: {}",
        start_time.elapsed().as_nanos(), cipher_text);
    //Sha3-512
    start_time = Instant::now();
    cipher_text = get_sha3_512(&plain_text);
    println!("Sha3-512 time: {}ns Encrypted: {}",
        start_time.elapsed().as_nanos(), cipher_text);
    //Md5
    start_time = Instant::now();
    cipher_text = get_md5(&plain_text);
    println!("Md5 time: {}ns Encrypted: {}",
        start_time.elapsed().as_nanos(), cipher_text);
}

fn get_md5(text: &String) -> String {
    use md5::{Md5, Digest};
    let mut hasher = Md5::new();
    hasher.update(text.as_bytes())::::;
    let buf = hasher.finalize();
    return String::from_utf8_lossy(buf.as_slice()).into_owned();
}

fn get_sha3_512(text: &String) -> String {
    let mut hasher = Sha3::sha3_512();
    hasher.input_str(text.as_str());
    return hasher.result_str();
}

fn get_sha3_256(text: &String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(text.as_str());
    return hasher.result_str();
}

fn get_sha2_512(text: &String) -> String {
    let mut hasher = Sha512::new();
    hasher.input_str(text.as_str());
    return hasher.result_str();
}

fn get_sha2_256(text: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(text.as_str());
    return hasher.result_str();
}

fn get_sha1(text: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(text.as_str());
    return hasher.result_str();
}
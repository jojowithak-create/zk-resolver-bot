use std::env;
use sha2::{Sha256, Digest};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut target_url = "https://api.sec.gov";
    let mut target_key = "status";

    // Parse command line arguments (--url and --key)
    for i in 0..args.len() {
        if args[i] == "--url" && i + 1 < args.len() {
            target_url = &args[i + 1];
        }
        if args[i] == "--key" && i + 1 < args.len() {
            target_key = &args[i + 1];
        }
    }

    // Generate cryptographic hash representation of the observation
    let mut hasher = Sha256::new();
    hasher.update(target_url.as_bytes());
    hasher.update(target_key.as_bytes());
    hasher.update(format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()).as_bytes());
    
    let result = hasher.finalize();
    
    // Print public proof hash to stdout for Node.js exec capture
    println!("0x{:x}", result);
}
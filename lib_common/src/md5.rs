use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hasher};
use std::fmt::Write;

fn proof_of_work(message: &str, complexity: u32) -> (u32, String) {
    let mut rng = rand::thread_rng();
    let mut seed: u32;
    let mut hasher = DefaultHasher::new();
    let mut hash: u64;

    loop {
        seed = rng.gen();
        hasher.write(message.as_bytes());
        hasher.write(&seed.to_be_bytes());
        hash = hasher.finish();

        let mut leading_zeros = 0;
        for byte in hash.to_be_bytes().iter() {
            if *byte == 0 {
                leading_zeros += 8;
            } else {
                let mut mask = 0b10000000;
                while mask > 0 && (*byte & mask) == 0 {
                    leading_zeros += 1;
                    mask >>= 1;
                }
                break;
            }
        }

        if leading_zeros >= complexity {
            break;
        } else {
            hasher = DefaultHasher::new();
        }
    }

    let mut hash_str = String::new();
    for byte in hash.to_be_bytes().iter() {
        write!(hash_str, "{:02x}", byte).unwrap();
    }

    (seed, hash_str)
}





#[test]
fn test_sqrt()  {
    let message = "hello";
    let complexity = 9;
    let (seed, hash) = proof_of_work(message, complexity);
    println!("seed: {:x}", seed);
    println!("hash: {}", hash);
}
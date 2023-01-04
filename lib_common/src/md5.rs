use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::fmt::Write;
use crate::message::{MD5HashCashInput, MD5HashCashOutput};

pub fn proof_of_work(message: MD5HashCashInput) -> MD5HashCashOutput {
    let mut seed: u64 = 0;
    let mut hasher = DefaultHasher::new();
    let mut hash: u64;

    loop {
        hasher.write(message.message.as_bytes());
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

        if leading_zeros >= message.complexity {
            break;
        } else {
            seed += 1;
            hasher = DefaultHasher::new();
        }
    }

    let mut hash_str = String::new();
    for byte in hash.to_be_bytes().iter() {
        write!(hash_str, "{:02x}", byte).unwrap();
    }

    MD5HashCashOutput{ seed, hashcode: hash_str }
}





#[test]
fn test_sqrt()  {
    let message = "hello";
    let complexity = 9;
    let test = MD5HashCashInput{ complexity: complexity, message: message.to_string() };
    let result = proof_of_work(test);
    let mut hasher = DefaultHasher::new();
    hasher.write(message.as_bytes());
    hasher.write(&result.seed.to_be_bytes());
    let calculated_hash = hasher.finish();
    let hash_u64 = u64::from_str_radix(&*result.hashcode, 16).unwrap();
    assert_eq!(hash_u64, calculated_hash)

}
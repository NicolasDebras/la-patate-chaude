use md5::Digest;
use crate::message::{MD5HashCashInput, MD5HashCashOutput};
extern crate md5;



pub fn proof_of_work(input: MD5HashCashInput) -> MD5HashCashOutput {
    let mut seed: u64 = 0;
    let mut digest: Digest = md5::compute(format!("{:016X}{}", seed, input.message));
    while check_number_of_bit_at_zero(digest.as_slice(), input.complexity) == false {
        seed += 1;
        digest = md5::compute(format!("{:016X}{}", seed, input.message));
    }
    return MD5HashCashOutput { seed, hashcode: format!("{:032X}", digest) };
}

fn check_number_of_bit_at_zero(number: &[u8], expected_of_zero: u32) -> bool {

    let mut number_as_bits: u128 = 0;
    number_as_bits = number[0] as u128;
    for i in 1..number.len() {
        // decale les nombre des bits vers la gauches de 8 positions
        number_as_bits = number_as_bits << 8;
        number_as_bits += number[i] as u128;
    }
    number_as_bits = number_as_bits.reverse_bits();
    let mut number_of_zero = 0;
    while number_of_zero < expected_of_zero {
        if (number_as_bits & 0x1) == 0 {
            number_of_zero += 1;
        } else {
            return false;
        }
        number_as_bits = number_as_bits >> 1;
    }
    return true;
}

pub fn number_of_zeros(number: &[u8;16])-> i32{
    let mut number_as_bits: u128 = 0;
    number_as_bits = number[0] as u128;
    for i in 1..number.len() {
        // decale les nombre des bits vers la gauches de 8 positions
        number_as_bits = number_as_bits << 8;
        number_as_bits += number[i] as u128;
    }
    number_as_bits = number_as_bits.reverse_bits();
    let mut number_of_zero = 0;
    for _n in  0 .. 128 {
        if (number_as_bits & 0x1) == 0 {
            number_of_zero += 1;
        }
        number_as_bits = number_as_bits >> 1;
    }
    number_of_zero
}

#[test]
fn test_check_number_of_bit_at_zero() {

    let input = MD5HashCashInput{
        complexity: 0,
        message: "hello".to_string(),
    };
    let input2 = MD5HashCashInput{
        complexity: 0,
        message: "hellO".to_string(),
    };
    let test = proof_of_work(input);
    let hello = proof_of_work(input2);
    assert_ne!(test.hashcode , hello.hashcode )
}



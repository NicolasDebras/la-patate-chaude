use lib_common::message::{RecoverSecretInput, RecoverSecretOutput};
use lib_common::recovery_secret::RS;
use lib_common::md5::MD5;
use lib_common::challenge::Challenge;
use lib_common::message::MD5HashCashInput;
use lib_common::message::MD5HashCashOutput;

//fichier de test de recovery secret

#[test]
fn test_correct_c_est_chou() {
    let rsInput = RecoverSecretInput {
        word_count: 1,
        letters: "t cCehuCethoCeschouC'schout h".to_string(),
        tuple_sizes: vec![3, 4, 5, 7, 7, 3]
    };
    let rsCorrect = RecoverSecretOutput {
        secret_sentence: "C'est Chou".to_string()
    };
    let test = RS::new(rsInput);
    assert!(test.verify(&rsCorrect))
}

#[test]
fn test_md5(){
    let md5= MD5HashCashInput{
        complexity : 100,
        message: "hello".to_string()
    };
    let md5Result = MD5HashCashOutput{
        seed: 844,
        hashcode: "00441745D9BDF8E5D3C7872AC9DBB2C3".to_string(),
    };
    let problem= MD5::new(md5);
    assert!(problem.verify(&md5Result));
}
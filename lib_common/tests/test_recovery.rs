use lib_common::challenge::Challenge;
use lib_common::md5::MD5;
use lib_common::message::MD5HashCashInput;
use lib_common::message::MD5HashCashOutput;
use lib_common::message::{RecoverSecretInput, RecoverSecretOutput};
use lib_common::recovery_secret::RS;

///fichier de test de recovery secret et de md5 hashcash

#[test]
fn test_correct_c_est_chou() {
    let rs_input = RecoverSecretInput {
        word_count: 1,
        letters: "t cCehuCethoCeschouC'schout h".to_string(),
        tuple_sizes: vec![3, 4, 5, 7, 7, 3],
    };
    let rs_correct = RecoverSecretOutput {
        secret_sentence: "C'est Chou".to_string(),
    };
    let test = RS::new(rs_input);
    assert!(test.verify(&rs_correct))
}

#[test]
fn test_md5() {
    let md5 = MD5HashCashInput {
        complexity: 2,
        message: "hello".to_string(),
    };
    let md5_result = MD5HashCashOutput {
        seed: 844,
        hashcode: "00441745D9BDF8E5D3C7872AC9DBB2C3".to_string(),
    };
    let test = MD5::new(md5);
    test.solve();
    let result = test.verify(&md5_result);
    assert_eq!(result, true);
}

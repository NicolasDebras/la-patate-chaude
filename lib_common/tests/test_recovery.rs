use lib_common::message::{RecoverSecretInput, RecoverSecretOutput};
use lib_common::recovery_secret::RS;
use lib_common::md5::MD5;

//fichier de test de recovery secret

#[test]
fn test_correct_c_est_chou() {
    let rsInput = RecoverSecretInput {
        word_count: 1,
        letters: "t cCehuCethoCeschouC'schout h",
        tuple_sizes: vec![3, 4, 5, 7, 7, 3]
    };
    let rsCorrect = RecoverSecretOutput {
        secret_sentence: "C'est Chou"
    };
    let test = RS::new(rsInput);
    assert!(test.verify(&rsCorrects))
}
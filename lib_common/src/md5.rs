extern crate md5;

use crate::challenge::Challenge;
use crate::message::{MD5HashCashInput, MD5HashCashOutput};

/// La structure `MD5` représente un défi à résoudre à l'aide de la fonction de hachage MD5.
pub struct MD5 {
    pub input: MD5HashCashInput,
}

/// Implémentation du défi `MD5HashCash`.
impl Challenge for MD5 {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    /// Retourne le nom du défi.
    fn name() -> String {
        "MD5HashCash".to_string()
    }

    /// Crée une nouvelle instance du défi de la fonction de hachage MD5.
    fn new(input: Self::Input) -> Self {
        MD5 { input }
    }

    /// Résout le défi de la fonction de hachage MD5 en trouvant une valeur de départ qui, une fois concaténée
    /// avec le message d'entrée et haché à l'aide de la fonction de hachage MD5, produit un code de hachage
    /// avec un certain nombre de zéros non significatifs.
    ///
    /// # Retour
    ///
    /// La solution au défi de la fonction de hachage MD5 en tant que structure `MD5HashCashOutput`.
    ///

    fn solve(&self) -> Self::Output {
        let mut answer = Self::Output {
            seed: 0,
            hashcode: "".to_string(),
        };

        for seed in 0..=u64::MAX {
            let input = format!("{seed:0>16X}{}", self.input.message);
            let hashcode = format!("{:0>16X}", md5::compute(&input));
            let num_hashcode = u128::from_str_radix(&hashcode, 16).unwrap();

            let zeros = num_hashcode.leading_zeros();
            if zeros >= self.input.complexity {
                answer = Self::Output { seed, hashcode };
                break;
            }
        }
        answer
    }

    /// Vérifie une solution au défi de la fonction de hachage MD5 en vérifiant que la solution
    /// valeur de départ, concaténée avec le message d'entrée et hachée à l'aide de la fonction de hachage MD5,
    /// produit un code de hachage avec le nombre requis de zéros non significatifs, et que le code de hachage
    /// correspond à celui fourni dans la solution.
    ///
    /// # Arguments
    ///
    /// * `answer` - La solution à vérifier, en tant que structure `MD5HashCashOutput`.
    ///
    /// # Retour
    ///
    /// `true` si la solution est valide, `false` sinon.
    ///

    fn verify(&self, answer: &Self::Output) -> bool {
        let seed = answer.seed;
        let input = format!("{seed:0>16X}{}", self.input.message);
        let hashcode = format!("{:X}", md5::compute(&input));
        count_bits_to_zero(&hashcode) >= self.input.complexity && answer.hashcode == hashcode
    }
}

/// Compte le nombre de zéros non significatifs dans un code de hachage MD5.
fn count_bits_to_zero(hex_string: &str) -> u32 {
    let hex_value = u128::from_str_radix(hex_string, 16).unwrap();
    hex_value.leading_zeros()
}

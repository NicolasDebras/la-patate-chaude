use crate::challenge::Challenge;
use crate::message::{RecoverSecretInput, RecoverSecretOutput};
use std::collections::HashMap;

/// Le challenge `RecoverSecret` est un challenge de type `Challenge`.
pub struct RS {
    pub input: RecoverSecretInput,
}

/// L'implémentation du challenge `RecoverSecret`.
impl Challenge for RS {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    ///retourne le nom du challenge
    fn name() -> String {
        "RecoverSecret".to_string()
    }
    /// Crée une nouvelle instance du défi de recover secret.
    fn new(input: Self::Input) -> Self {
        RS { input }
    }

    /// Cette fonction prend en entrée un tableau de chaînes de caractères `tab` et renvoie une chaîne de caractères qui représente le secret caché dans les éléments de `tab`.
    ///
    /// # Arguments
    ///
    /// * `tab` - un vecteur de chaînes de caractères contenant des éléments cachant un secret.
    ///
    /// ```
    fn solve(&self) -> Self::Output {
        let tab = create_element_tuple(self.input.letters.clone(), self.input.tuple_sizes.clone());
        let res = recover_secret(tab);
        return Self::Output {
            secret_sentence: res,
        };
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        // Vérifie que chaque lettre apparaît dans l'ordre relatif correct.
        let mut last_seen: HashMap<char, usize> = HashMap::new();
        for car in answer.secret_sentence.chars() {
            let tuple_index =
                create_element_tuple(self.input.letters.clone(), self.input.tuple_sizes.clone())
                    .iter()
                    .position(|tuple| tuple.contains(car))
                    .unwrap();
            let car_index =
                create_element_tuple(self.input.letters.clone(), self.input.tuple_sizes.clone())
                    [tuple_index]
                    .chars()
                    .position(|c| c == car)
                    .unwrap();
            if let Some(prev_index) = last_seen.get(&car) {
                if tuple_index != *prev_index || car_index != (*prev_index + 1) {
                    // Si la lettre n'est pas dans l'ordre relatif correct, renvoie `false`.
                    return false;
                }
            }
            last_seen.insert(car, tuple_index);
        }

        return has_unique_chars(answer.secret_sentence.clone());
    }
}

/// Etant donné une liste de chaînes représentant des ordres partiels de caractères, cette fonction récupère
/// l'ordre complet des caractères. La liste d'entrée est un Vec<String> où chaque chaîne
/// représente un ordre partiel des caractères. Par exemple, l'entrée ["aew", "vwy", "ywd"]
/// représente l'ordre partiel 'a' < 'e' < 'w', 'v' < 'w' < 'y' et 'y' < 'w' < 'd'. Le résultat
/// est une chaîne représentant l'ordre complet des caractères, par exemple "aevwyd".
///
/// ```
fn recover_secret(tab: Vec<String>) -> String {
    // Initialisation d'un tableau vide pour stocker les résultats.
    let mut res = Vec::new();
    // Boucle sur tous les éléments du tableau
    for element_tuple in tab {
        // Boucle sur les caractères de chaque élément.
        for car in element_tuple.chars() {
            //println!("test car : {:?}", car);
            // Récupération de l'index du caractère actuel.
            let idx = element_tuple.chars().position(|x| x == car).unwrap();
            println!("index {:?}", idx);
            println!("tuple {:?}", element_tuple);
            println!("car {:?}", car);
            // Si l'index est supérieur ou égal à 0 (
            if idx > 0 {
                // Si le caractère précédent est dans `res` et que le caractère actuel n'y est pas.
                if res.contains(&element_tuple.chars().nth(idx - 1).unwrap())
                    && res.contains(&car) == false
                {
                    // Récupération de l'index du caractère précédent dans `res`.
                    let index = res
                        .iter()
                        .position(|x| x == &element_tuple.chars().nth(idx - 1).unwrap())
                        .unwrap();
                    // Insertion du caractère actuel après le caractère précédent dans `res`.
                    res.insert(index + 1, car);
                    println!("test 1 {:?}", res);
                }
                // Si le caractère actuel n'est pas dans `res`.
                else if res.contains(&car) == false {
                    // Insertion du caractère actuel au début de `res`.
                    res.insert(0, car);
                    println!("test 2 {:?}", res);
                }
                // Si l'index + 1 est inférieur à la longueur de l'élément.
                if idx + 1 < element_tuple.len() {
                    // Récupération des index du caractère suivant et du caractère actuel dans `res`.
                    let a = res
                        .iter()
                        .position(|x| x == &element_tuple.chars().nth(idx + 1).unwrap_or_default());
                    let b = res
                        .iter()
                        .position(|x| x == &element_tuple.chars().nth(idx).unwrap_or_default());
                    // Si le caractère suivant est dans `res` et que son index est inférieur à celui du caractère actuel.
                    if res.contains(&element_tuple.chars().nth(idx + 1).unwrap()) && a < b {
                        // Récupération de l'index du caractère suivant dans `res`.
                        let x = res
                            .iter()
                            .position(|x| x == &element_tuple.chars().nth(idx + 1).unwrap());
                        // Suppression du caractère suivant de `res`.
                        res.remove(x.unwrap_or_default());
                        //res.insert(x)
                        println!("test 3 {:?}", res);
                    }
                }
            } else {
                if res.contains(&car) == false {
                    res.insert(0, car);
                    println!("test 1.5 {:?}", res);
                }
            }
        }
    }
    res.iter().collect()
}

///fonction qui permet de créer un tableau de string corresponds au chaine entré en paramètre et tuple size
fn create_element_tuple(letters: String, element_tuple_sizes: Vec<usize>) -> Vec<String> {
    let mut i = 0;
    let mut tab = Vec::new();
    for element in element_tuple_sizes {
        tab.push((&letters[i..i + element]).to_string());
        i = i + element;
    }
    tab
}

//onction qui verifie si pas de doublon dans un string
fn has_unique_chars(s: String) -> bool {
    let mut seen_chars = [false; 256];
    for c in s.chars() {
        let c_val = c as usize;
        if seen_chars[c_val] {
            return false;
        }
        seen_chars[c_val] = true;
    }
    true
}

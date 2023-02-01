use crate::message::{RecoverSecretInput, RecoverSecretOutput};
use crate::challenge::Challenge;
extern crate recovery_secret;


pub struct RS {
    pub input: RecoverSecretInput
} 

impl Challenge for RS {

    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    //retourne le nom du challenge
    fn name() -> String {
        "RecoverSecret".to_string();
    }

    fn new(input: Self::Input) -> Self {
        RS { input }
    }
    
    fn solve(&self) -> Self::Output {
        let tab = create_element_tuple(self.input.letters, self.input.tuple_sizes);
        let res = recover_secret(tab);
        return Self::Output {
            res
        }
    }

    //a faire plus tard 
    fn verify(&self, answer: &Self::Output) -> bool {
        return true;
    }

}


// La fonction `recover_secret` prend en entrée un tableau de chaînes et renvoie une chaîne.
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
                if res.contains(&element_tuple.chars().nth(idx-1).unwrap()) && res.contains(&car) == false {
                    // Récupération de l'index du caractère précédent dans `res`.
                    let index = res.iter().position(|x| x == &element_tuple.chars().nth(idx-1).unwrap()).unwrap();
                    // Insertion du caractère actuel après le caractère précédent dans `res`.
                    res.insert(index+1, car);
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
                    let a = res.iter().position(|x| x == &element_tuple.chars().nth(idx+1).unwrap_or_default());
                    let b = res.iter().position(|x| x == &element_tuple.chars().nth(idx).unwrap_or_default());
                    // Si le caractère suivant est dans `res` et que son index est inférieur à celui du caractère actuel.
                    if res.contains(&element_tuple.chars().nth(idx+1).unwrap()) && a < b {
                        // Récupération de l'index du caractère suivant dans `res`.
                        let x = res.iter().position(|x| x == &element_tuple.chars().nth(idx+1).unwrap());
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

//fonction qui permet de créer un tableau de string corresponds au chaine entré en paramètre et tuple size
fn create_element_tuple(letters: String, element_tuple_sizes: Vec<usize>) -> Vec<String> {
    let mut i = 0;
    let mut tab = Vec::new();
    for element in element_tuple_sizes { 
        tab.push((&letters[i..i+element]).to_string());    
        i = i + element;
    }
    tab
}

//manque la méthode de gestion de la classe 
//prend en parametre la struct et en renvoie le message complété

// fn main() {
//     let letters = "t cCehuCethoCeschouC'schout h";
//     let element_tuple_sizes = vec![3, 4, 5, 7, 7, 3];
//     let tab = create_element_tuple(letters.to_string(), element_tuple_sizes); // ca marche jusqu'a la 
//     println!("{:?}", tab);
//     println!("{}", recover_secret(tab));

// }
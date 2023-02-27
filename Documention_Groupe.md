# Documentation

* Aristide Fumo a fait le serveur, client et a aidé à faire le challenge md5 et l'intégrer dans le client et serveur
* Nicolas Debras a fait le challenge et son intégration dans le côté client complexité max 15 et l'intégration en github
  action et les tests unitaires
* TEVOT Kisseime a fait la première itération sur le challenge md5

## Comment compiler le projet

### Partie client rust et serveur codé par le professeur

#### Pour md5-hash-cash

```bash
./server  --monitor
cargo  run --bin  client name localhost
cargo  run --bin  client name2 localhost
./start_server; ./start_server
```

#### Pour recover-secret

le recover secret ne marche avec une complexité max de 15

```bash
./server -g  recover-secret -c 2 --monitor
cargo  run --bin  client name localhost
cargo  run --bin  client name2 localhost
./start_server; ./start_server
```

### Partie serveur et client rust

```bash

Pour la partie client et serveur qu'on a codé en rust

```bash
cargo run --bin  server md5-hash-cash
cargo  run --bin  client name localhost
```

Le serveur ne marche qu'avec un client. Il ne marche pas avec plusieurs clients.
Une itération sur une branch mais n'a pas été convaincant

### Partie documentation

Pour voir la documentation du projet

```bash
cargo doc --open
```

### Lib en plus

lipsum = "0.8"

Cette librairie permet de générer des phrases aléatoires en lips um.
Grâce à cela nous avons le moyen de générer des phrases aléatoires pour le challenge md5-hash-cash.

Pour la complexity est généré aléatoirement entre 0 et le nombre de mots réussi.

### Bonus en plus

## Bonus possibles :

* Réaliser une interface pour le client et/ou le serveur

Fait un serveur et un client en rust

* Ajouter une intégration continue qui permette de tester votre code client et serveur (sous GitHub ou GitLab)

  Nous avons un fichier .yml qui permet de faire les tests unitaires et de compiler le projet qui est dans
  .github/workflows/rust.yml


* Déployer des techniques avancées pour optimiser la performance de résolution du challenge pour le md5-hash-cash

  Nous avons récrit le challenge, car il n'était pas optimisé.

* Réduire au maximum (voire à zéro) les éléments suivants

  (ce qui est un élément très qualitatif pour vos codes en Rust en plus d'être un bonus dans le cadre de ce projet)
    * les `unwrap()`, les `expect()`, les `panic!()`
    * les `mut` (variables mutables)
    * less *warnings* de compilation

Nous avons essayé le plus possible de supprimer les erreurs de compilation et les warnings
et les erreurs de panic. Par exemple le client est géré si s'appelle deux fois avec le même nom.
Ou quand il a un timeout du niveau du serveur.
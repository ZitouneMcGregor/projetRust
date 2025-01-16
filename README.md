# Moteur de Recherche Documentaire

Ce projet implémente un mini moteur de recherche documentaire basé sur la **similarité cosinus**.  
Il comporte plusieurs entités principales :  

- **`Database`** : gère un ensemble de collections nommées  
- **`Collection`** : représente un ensemble de documents, chacun ayant un `DocumentId` et un vecteur (généralement une représentation numérique du contenu)  
- **`cosine_similarity`** : calcule la similarité cosinus entre deux vecteurs  

## Sommaire

1. [Fonctionnalités](#fonctionnalités)  
2. [Prérequis](#prérequis)  
3. [Installation](#installation)  
4. [Utilisation](#utilisation)  
5. [Documentation](#documentation)  
6. [Licence](#licence)  

---

## Fonctionnalités

- **Ajouter ou créer une collection** : via la méthode `Database::add_collection`.  
- **Ajouter ou modifier des documents** : via la méthode `Collection::add_or_update`.  
- **Rechercher un document** : en passant une requête (un `Vec<f32>`) à la méthode `Collection::search` ou à la méthode `Database::search_in_collection`.  
- **Calcul parallèle** : le produit scalaire et les magnitudes sont calculés dans des threads séparés pour illustrer la programmation concurrente.

---

## Prérequis

- **Rust** : version stable (1.60+ de préférence).  
- **Cargo** : l’outil de gestion de projets Rust.  
- **Bibliothèque [uuid](https://crates.io/crates/uuid)** : pour générer des identifiants uniques (`DocumentId`).  
- **Bibliothèque [colored](https://crates.io/crates/colored)** : pour la mise en forme du texte dans le terminal (non essentiel au fonctionnement, purement visuel).

---

## Installation

1. **Cloner le dépôt** :  
   ```bash
   git clone https://github.com/ZitouneMcGregor/projetRust
   cd projetRust
   ```
2. **Compiler le projet** :  
   ```bash
   cargo build
   ```
3. **Exécuter le projet** :  
   ```bash
   cargo run
   ```

---

## Utilisation

Une fois le projet exécuté (avec `cargo run`), vous verrez dans la console un exemple de création de deux collections (*NotaryDocuments* et *LegalFiles*), l’ajout de documents, puis une recherche s’effectuant avec un vecteur de requête `[1.0, 1.0, 1.0]`.

Par exemple :

```
=== Bienvenue dans le Moteur de Recherche Documentaire ===

Ajout des collections...
Ajout de documents à la collection 'NotaryDocuments'...
Documents ajoutés avec succès !

Ajout de documents à la collection 'LegalFiles'...
Documents ajoutés avec succès !

=== Recherche avec la requête: [1.0, 1.0, 1.0] ===

Résultats de recherche dans 'NotaryDocuments':
Document ID: 123e4567-e89b-12d3-a456-426614174000 - Similarité: 0.9746
...
```

Vous verrez ensuite les **trois** documents les plus similaires (car le `k` est défini à 3) s’afficher pour chacune des collections.

---

## Documentation

Le projet inclut une documentation interne conforme aux standards *Rustdoc*.  
Vous pouvez la générer avec :

```bash
cargo doc --open
```

Cela ouvrira dans votre navigateur une page listant toutes les structures et fonctions avec leurs descriptions détaillées.

---

## Licence

Ce projet est distribué sous les termes de la licence **ZitIndustries**.  
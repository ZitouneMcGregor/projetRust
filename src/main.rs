use std::collections::HashMap;
use std::thread;
use uuid::Uuid;

/// # Type: `DocumentId`
///
/// `DocumentId` est un alias pour [`Uuid`].  
/// Il représente l'identifiant unique d'un document.
type DocumentId = Uuid;

/// # Type: `Vector`
///
/// `Vector` est un alias pour un `Vec` contenant des couples `(DocumentId, f32)`.  
/// Il est utilisé pour représenter un ensemble de résultats de recherche (par exemple, un score de similarité associé à un identifiant de document).
type Vector = Vec<(DocumentId, f32)>;

/// # Type: `SearchResult`
///
/// `SearchResult` est un alias de [`Vector`].
/// Il désigne la liste finale de résultats d'une recherche.
type SearchResult = Vector;

/// # Structure: `Collection`
///
/// `Collection` gère un ensemble de documents, identifiés par [`DocumentId`], et stocke leurs vecteurs (par exemple, leur représentation numérique).
/// Elle offre des méthodes pour ajouter, mettre à jour, supprimer et rechercher des documents.
struct Collection {
    /// Les données de la collection stockées sous forme de clé-valeur (`DocumentId`, vecteur).
    data: HashMap<DocumentId, Vec<f32>>,
}

impl Collection {
    /// Crée une nouvelle instance de [`Collection`].
    ///
    /// # Exemple
    ///
    /// ```
    /// let collection = Collection::new();
    /// ```
    fn new() -> Self {
        Collection {
            data: HashMap::new(),
        }
    }

    /// Ajoute ou met à jour le vecteur associé à un [`DocumentId`].
    ///
    /// # Paramètres
    /// - `key`: L'identifiant unique du document.
    /// - `vector`: Le vecteur associé au document (ex. représentation sémantique).
    ///
    /// # Exemple
    ///
    /// ```
    /// let mut collection = Collection::new();
    /// let doc_id = Uuid::new_v4();
    /// collection.add_or_update(doc_id, vec![1.0, 2.0, 3.0]);
    /// ```
    fn add_or_update(&mut self, key: DocumentId, vector: Vec<f32>) {
        self.data.insert(key, vector);
    }

    /// Récupère le vecteur associé à un [`DocumentId`], s'il existe.
    ///
    /// # Paramètres
    /// - `key`: La référence à l'identifiant unique du document.
    ///
    /// # Retour
    /// - `Option<&Vec<f32>>`: Le vecteur si le document est trouvé, ou `None` sinon.
    ///
    /// # Exemple
    ///
    /// ```
    /// let doc = collection.get(&doc_id);
    /// if let Some(vector) = doc {
    ///     // Utiliser le vecteur
    /// }
    /// ```
    #[allow(unused)]
    fn get(&self, key: &DocumentId) -> Option<&Vec<f32>> {
        self.data.get(key)
    }

    /// Supprime le document (et son vecteur) associé à un [`DocumentId`].
    ///
    /// # Paramètres
    /// - `key`: La référence à l'identifiant unique du document.
    ///
    /// # Exemple
    ///
    /// ```
    /// collection.remove(&doc_id);
    /// ```
    #[allow(unused)]
    fn remove(&mut self, key: &DocumentId) {
        self.data.remove(key);
    }

    /// Recherche les documents les plus proches d'une requête donnée en utilisant la **similarité cosinus**.
    ///
    /// # Paramètres
    /// - `query`: Le vecteur représentant la requête de recherche.
    /// - `k`: Le nombre maximal de résultats à retourner.
    ///
    /// # Retour
    /// - [`SearchResult`]: Une liste de paires (`DocumentId`, score_de_similarité) classées par ordre décroissant de similarité.
    ///
    /// # Exemple
    ///
    /// ```
    /// let results = collection.search(&[1.0, 1.0, 1.0], 3);
    /// for (doc_id, similarity) in results {
    ///     println!("DocID: {}, Similarité: {}", doc_id, similarity);
    /// }
    /// ```
    fn search(&self, query: &[f32], k: usize) -> SearchResult {
        let mut results: SearchResult = self
            .data
            .iter()
            .filter_map(|(key, vector)| {
                // On ignore les documents dont la dimension du vecteur ne correspond pas à la requête
                if vector.len() != query.len() {
                    return None;
                }
                let similarity = cosine_similarity(query, vector);
                Some((*key, similarity))
            })
            .collect();

        // Tri par ordre décroissant de la similarité
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.into_iter().take(k).collect()
    }
}

/// # Structure: `Database`
///
/// `Database` gère un ensemble de collections (instances de [`Collection`]) identifiées par un nom (`String`).
/// Elle offre des méthodes pour ajouter une collection, obtenir une collection (en lecture seule ou mutable) et effectuer des recherches.
struct Database {
    /// Les collections stockées sous forme de clé-valeur (`String`, [`Collection`]).
    collections: HashMap<String, Collection>,
}

impl Database {
    /// Crée une nouvelle instance de [`Database`].
    ///
    /// # Exemple
    ///
    /// ```
    /// let db = Database::new();
    /// ```
    fn new() -> Self {
        Database {
            collections: HashMap::new(),
        }
    }

    /// Ajoute une nouvelle [`Collection`] dans la base de données.
    ///
    /// # Paramètres
    /// - `name`: Le nom de la collection (unique).
    ///
    /// # Exemple
    ///
    /// ```
    /// let mut db = Database::new();
    /// db.add_collection("NotaryDocuments".to_string());
    /// ```
    fn add_collection(&mut self, name: String) {
        self.collections.insert(name, Collection::new());
    }

    /// Récupère une [`Collection`] en lecture seule depuis la base de données, si elle existe.
    ///
    /// # Paramètres
    /// - `name`: Le nom de la collection.
    ///
    /// # Retour
    /// - `Option<&Collection>`: La collection si elle est trouvée, ou `None` sinon.
    ///
    /// # Exemple
    ///
    /// ```
    /// if let Some(collection) = db.get_collection("NotaryDocuments") {
    ///     // Utiliser la collection
    /// }
    /// ```
    #[allow(unused)]
    fn get_collection(&self, name: &str) -> Option<&Collection> {
        self.collections.get(name)
    }

    /// Récupère une [`Collection`] en écriture depuis la base de données, si elle existe.
    ///
    /// # Paramètres
    /// - `name`: Le nom de la collection.
    ///
    /// # Retour
    /// - `Option<&mut Collection>`: La collection si elle est trouvée, ou `None` sinon.
    ///
    /// # Exemple
    ///
    /// ```
    /// if let Some(collection) = db.get_collection_mut("NotaryDocuments") {
    ///     // Ajouter ou modifier des documents
    /// }
    /// ```
    fn get_collection_mut(&mut self, name: &str) -> Option<&mut Collection> {
        self.collections.get_mut(name)
    }

    /// Effectue une recherche dans une [`Collection`] spécifiée par son nom.
    ///
    /// # Paramètres
    /// - `collection_name`: Le nom de la collection dans laquelle effectuer la recherche.
    /// - `query`: Le vecteur de la requête.
    /// - `k`: Le nombre de résultats maximal à retourner.
    ///
    /// # Retour
    /// - `Option<SearchResult>`: Les résultats de recherche (liste de (`DocumentId`, score_de_similarité)) si la collection est trouvée, `None` sinon.
    ///
    /// # Exemple
    ///
    /// ```
    /// let query = vec![1.0, 1.0, 1.0];
    /// if let Some(results) = db.search_in_collection("NotaryDocuments", &query, 3) {
    ///     for (doc_id, similarity) in results {
    ///         println!("DocID: {}, Similarité: {}", doc_id, similarity);
    ///     }
    /// }
    /// ```
    fn search_in_collection(&self, collection_name: &str, query: &[f32], k: usize) -> Option<SearchResult> {
        self.collections.get(collection_name).map(|collection| collection.search(query, k))
    }
}

/// Calcule la similarité cosinus entre deux vecteurs.
///
/// # Paramètres
/// - `vector1`: Le premier vecteur.
/// - `vector2`: Le second vecteur.
///
/// # Retour
/// - `f32`: La valeur de similarité cosinus entre les deux vecteurs, comprise entre -1.0 et 1.0.
///   Si l'une des normes est nulle, la fonction retourne 0.0.
///
/// # Exemple
///
/// ```
/// let similarity = cosine_similarity(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]);
/// println!("Similarité: {}", similarity);
/// ```
fn cosine_similarity(vector1: &[f32], vector2: &[f32]) -> f32 {
    let (dot_product, (magnitude1, magnitude2)) = parallel_calculate(vector1, vector2);

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        0.0
    } else {
        dot_product / (magnitude1 * magnitude2)
    }
}

/// Calcule le produit scalaire (dot product) et les normes des deux vecteurs en parallèle.
///
/// # Paramètres
/// - `vector1`: Le premier vecteur.
/// - `vector2`: Le second vecteur.
///
/// # Retour
/// - `(f32, (f32, f32))`: Un tuple contenant le produit scalaire, et le couple de normes (norme de `vector1`, norme de `vector2`).
///
/// # Exemple
///
/// ```
/// let (dot, (mag1, mag2)) = parallel_calculate(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]);
/// ```
fn parallel_calculate(vector1: &[f32], vector2: &[f32]) -> (f32, (f32, f32)) {

    // Clones nécessaires pour les threads
    let vector1_clone = vector1.to_vec();
    let vector2_clone = vector2.to_vec();
    let vector1_clone_again = vector1.to_vec();
    let vector2_clone_again = vector2.to_vec();
    
    // Calcul du produit scalaire dans un thread
    let dot_product_handle = thread::spawn(move || calculate_dot_product(vector1_clone, vector2_clone));
    // Calcul de la norme du premier vecteur dans un thread
    let magnitude1_handle = thread::spawn(move || calculate_magnitude(&vector1_clone_again));
    // Calcul de la norme du second vecteur dans un thread
    let magnitude2_handle = thread::spawn(move || calculate_magnitude(&vector2_clone_again));

    let dot_product = dot_product_handle.join().unwrap();
    let magnitude1 = magnitude1_handle.join().unwrap();
    let magnitude2 = magnitude2_handle.join().unwrap();

    (dot_product, (magnitude1, magnitude2))
}

/// Calcule le produit scalaire de deux vecteurs.
///
/// # Paramètres
/// - `vector1`: Le premier vecteur.
/// - `vector2`: Le second vecteur.
///
/// # Retour
/// - `f32`: La somme des multiplications coordonnées à coordonnées.
///
/// # Exemple
///
/// ```
/// let dot = calculate_dot_product(vec![1.0, 2.0], vec![3.0, 4.0]);
/// assert_eq!(dot, 11.0);
/// ```
fn calculate_dot_product(vector1: Vec<f32>, vector2: Vec<f32>) -> f32 {
    vector1.iter().zip(vector2).map(|(x, y)| x * y).sum()
}

/// Calcule la norme (euclidienne) d'un vecteur.
///
/// # Paramètres
/// - `vector`: Le vecteur à évaluer.
///
/// # Retour
/// - `f32`: La norme du vecteur.
///
/// # Exemple
///
/// ```
/// let mag = calculate_magnitude(&[3.0, 4.0]);
/// assert_eq!(mag, 5.0);
/// ```
fn calculate_magnitude(vector: &[f32]) -> f32 {
    vector.iter().map(|x| x * x).sum::<f32>().sqrt()
}

fn main() {
    use colored::*;

    // Création d'une nouvelle base de données
    println!("{}", "\n=== Bienvenue dans le Moteur de Recherche Documentaire ===\n".bold().truecolor(135, 206, 250));
    let mut db = Database::new();

    // Ajout des collections
    println!("{}", "Ajout des collections...".bold().bright_green());
    db.add_collection("NotaryDocuments".to_string());
    db.add_collection("LegalFiles".to_string());

    // Ajouter des documents dans "NotaryDocuments"
    if let Some(collection) = db.get_collection_mut("NotaryDocuments") {
        println!("{}", "\nAjout de documents à la collection 'NotaryDocuments'...".bold().yellow());
        collection.add_or_update(Uuid::new_v4(), vec![1.0, 2.0, 3.0]);
        collection.add_or_update(Uuid::new_v4(), vec![4.0, 5.0, 6.0]);
        println!("{}", "Documents ajoutés avec succès !".bright_green());
    }

    // Ajouter des documents dans "LegalFiles"
    if let Some(collection) = db.get_collection_mut("LegalFiles") {
        println!("{}", "\nAjout de documents à la collection 'LegalFiles'...".bold().yellow());
        collection.add_or_update(Uuid::new_v4(), vec![1.0, 0.0, 0.0]);
        collection.add_or_update(Uuid::new_v4(), vec![0.0, 1.0, 0.0]);
        println!("{}", "Documents ajoutés avec succès !".bright_green());
    }

    // Début de la recherche
    let query = vec![1.0, 1.0, 1.0];
    println!("\n{}", "=== Recherche avec la requête: [1.0, 1.0, 1.0] ===".bold().truecolor(255, 215, 0));

    // Recherche dans "NotaryDocuments"
    if let Some(results) = db.search_in_collection("NotaryDocuments", &query, 3) {
        println!("\n{}", "Résultats de recherche dans 'NotaryDocuments':".bright_blue().bold());
        for (key, similarity) in results {
            println!("{} {} {} {:.4}",
                "Document ID:".bright_magenta(), key.to_string().bright_white(), "- Similarité:".bright_magenta(), similarity);
        }
    } else {
        println!("{}", "Aucun résultat trouvé dans 'NotaryDocuments'.".red().bold());
    }

    // Recherche dans "LegalFiles"
    if let Some(results) = db.search_in_collection("LegalFiles", &query, 3) {
        println!("\n{}", "Résultats de recherche dans 'LegalFiles':".bright_blue().bold());
        for (key, similarity) in results {
            println!("{} {} {} {:.4}",
                "Document ID:".bright_magenta(), key.to_string().bright_white(), "- Similarité:".bright_magenta(), similarity);
        }
    } else {
        println!("{}", "Aucun résultat trouvé dans 'LegalFiles'.".red().bold());
    }

    // Fin
    println!("\n{}", "=== Fin de la recherche ===".bold().truecolor(135, 206, 250));
}

use rusqlite::{params, Connection, Result};
use dialoguer::{Select, Input};
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TransactionData {
    budget_name: String,
    desc: String,
    amount: i64,
}

pub struct BudgetManager {
    conn: Connection,
}

impl BudgetManager {
    pub fn new(db_name: &str) -> Result<Self> {
        let conn = Connection::open(db_name)?;

        // Créer les tables si elles n'existent pas
        conn.execute(
            "CREATE TABLE IF NOT EXISTS budgets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                amount INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                budget_id INTEGER NOT NULL,
                desc TEXT NOT NULL,
                amount INTEGER NOT NULL,
                FOREIGN KEY(budget_id) REFERENCES budgets(id)
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    // Fonction pour obtenir l'ID du budget
    pub fn get_budget_id(&self, budget_name: &str) -> Result<i64> {
        self.conn.query_row(
            "SELECT id FROM budgets WHERE name = ?1",
            params![budget_name],
            |row| row.get(0),
        )
    }

    // Fonction pour importer les transactions depuis un fichier CSV
    pub fn import_transactions(&self, file_path: &str) -> Result<()> {
        // Ouvrir le fichier CSV
        let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
        
        for result in rdr.deserialize() {
            // Chaque ligne du CSV est un `TransactionData`
            let record: TransactionData = result?;

            // Chercher l'ID du budget
            let budget_id = match self.get_budget_id(&record.budget_name) {
                Ok(id) => id,
                Err(_) => {
                    println!("Le budget '{}' n'existe pas.", record.budget_name);
                    continue; // Passer cette ligne et essayer avec la suivante
                }
            };

            // Ajouter la transaction à la base de données
            self.conn.execute(
                "INSERT INTO transactions (budget_id, desc, amount) VALUES (?1, ?2, ?3)",
                params![budget_id, record.desc, record.amount],
            )?;

            println!("Transaction ajoutée : {} - {} de {}", record.budget_name, record.desc, record.amount);
        }

        Ok(())
    }
}




fn main() -> Result<()> {
    let db_name = "budget_manager.db";
    let budget_manager = BudgetManager::new(db_name)?;

    // Demander à l'utilisateur de choisir une option dans un menu
    let options = vec!["Importer des transactions", "Quitter"];
    let selection = Select::new()
        .with_prompt("Que souhaitez-vous faire ?")
        .items(&options)
        .default(0)
        .interact()?;

    match selection {
        0 => {
            // Demander à l'utilisateur de saisir le chemin du fichier CSV
            let file_path: String = Input::new()
                .with_prompt("Entrez le chemin du fichier CSV")
                .interact_text()?;

            // Appeler la fonction d'importation avec le chemin du fichier
            budget_manager.import_transactions(&file_path)?;

            println!("Transactions importées avec succès !");
        }
        1 => {
            println!("Au revoir !");
        }
        _ => {}
    }

    Ok(())
}



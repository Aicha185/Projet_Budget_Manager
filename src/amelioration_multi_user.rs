use dialoguer::Input;
use rusqlite::{params, Connection, Result};
use std::error::Error;

pub struct BudgetManager {
    conn: Connection,
}

impl BudgetManager {
    // Initialisation de la base de données
    pub fn new(db_name: &str) -> Result<Self> {
        let conn = Connection::open(db_name)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS budgets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                amount INTEGER NOT NULL,
                FOREIGN KEY(user_id) REFERENCES users(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                budget_id INTEGER NOT NULL,
                desc TEXT NOT NULL,
                amount INTEGER NOT NULL,
                FOREIGN KEY(budget_id) REFERENCES budgets(id),
                FOREIGN KEY(user_id) REFERENCES users(id)
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    // Ajouter un utilisateur ou retourner son ID s'il existe déjà
    pub fn add_user(&self, user_name: &str) -> Result<i64> {
        // Tenter d'insérer l'utilisateur
        match self.conn.execute(
            "INSERT OR IGNORE INTO users (name) VALUES (?1)",
            params![user_name],
        ) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Erreur lors de l'insertion de l'utilisateur : {}", e);
                return Err(e);
            }
        };

        // Récupérer l'ID de l'utilisateur
        self.conn.query_row(
            "SELECT id FROM users WHERE name = ?1",
            params![user_name],
            |row| row.get(0),
        )
    }

    // Vérifier si un utilisateur existe
    pub fn user_exists(&self, user_name: &str) -> Result<bool> {
        let result: Result<i64> = self.conn.query_row(
            "SELECT id FROM users WHERE name = ?1",
            params![user_name],
            |row| row.get(0),
        );

        Ok(result.is_ok()) // Retourne true si l'utilisateur existe
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let db_name = "budget_manager.db";
    let budget_manager = BudgetManager::new(db_name)?;

    // Gestion des erreurs pour l'entrée utilisateur
    let user_name: String = loop {
        match Input::<String>::new()
            .with_prompt("Entrez votre nom d'utilisateur")
            .interact_text()
        {
            Ok(name) if !name.trim().is_empty() => break name,
            Ok(_) => println!("Le nom d'utilisateur ne peut pas être vide."),
            Err(_) => println!("Erreur lors de la lecture du nom. Réessayez."),
        }
    };

    // Vérifier ou ajouter l'utilisateur
    match budget_manager.add_user(&user_name) {
        Ok(user_id) => println!("Bienvenue, {}! (ID: {})", user_name, user_id),
        Err(e) => eprintln!("Impossible d'ajouter l'utilisateur : {}", e),
    }

    Ok(())
}

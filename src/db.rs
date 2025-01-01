use console::Style; 
use rusqlite::{params, Connection, Result};
use colored::*;
use prettytable::{Table, row, cell};
pub struct BudgetManager {
    conn: Connection,
}

impl std::fmt::Debug for BudgetManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BudgetManager").field("conn", &self.conn).finish()
    }
}

impl  BudgetManager {
   
    pub fn new(db_name: &str) -> Result<Self> {
        let conn = Connection::open(db_name)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS budgets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                budget_name TEXT NOT NULL,
                total_amount REAL NOT NULL,
                remaining_amount REAL NOT NULL 
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                budget_id INTEGER NOT NULL,
                transaction_name TEXT NOT NULL,
                amount REAL NOT NULL,
                FOREIGN KEY(budget_id) REFERENCES budgets(id)
            )",
            [],
        )?;

        Ok(Self { conn })
    }
 //==================================Function to add budgets====================================
    pub fn add_budget(&self, budget_name: &str, total_amount: f64) -> Result<()> {
        let error_style = Style::new().red();
        let success_style = Style::new().green();

         // Vérification : Le nom du budget ne peut pas être vide

        if budget_name.trim().is_empty(){ 
            println!("{}",error_style.apply_to("Erreur: Le champ nom du budget ne peut pas etre vide."));
            return Err(rusqlite::Error::InvalidParameterName("Le nom du budget est vide".into()));
        }
    
        // Vérification : Le montant total doit être positif et raisonnable

        if total_amount < 0.0 || total_amount > 1_000_000.0{ 
            println!("{}",error_style.apply_to("Erreur:  Le montant total doit être compris entre 0 et 1 000 000"));
            return Err(rusqlite::Error::InvalidParameterName("Le montant total est invalide".into()));
        }
        // Pour un nouveau budget, le montant restant est égal au montant total
        let remaining_amount = total_amount;
        
        // Insertion dans la base de données
        self.conn.execute(
            "INSERT INTO budgets (budget_name,total_amount,remaining_amount) VALUES (?1,?2,?3)",
             params! [budget_name,total_amount,remaining_amount],
        )?;
        println!("Ajout en cours...");
        println!("{}", success_style.apply_to(format!(
            "Budget '{}' ajouté avec succès ! Montant total : {:.2}, Montant restant : {:.2}.",
            budget_name, total_amount, remaining_amount )));
        Ok(())
    }

//==================================Function to remove budgets====================================
    pub fn remove_budget(&self, budget_name: &str) -> Result<()> {
        let error_style = Style::new().red();
        let success_style = Style::new().green();
        
        let rows_affected = self.conn.execute(
            "DELETE FROM budgets WHERE budget_name= ?1",
            params![budget_name],
        )?;

        if rows_affected > 0 {
            println!("{}", success_style.apply_to(format!("Budget'{}' supprimée avec succès .", budget_name)));
        } else {
            println!("{}", error_style.apply_to(format!("Le budget : '{}' n'existe pas.", budget_name)));
        }

        Ok(())
    }
//==================================Function to edit budgets====================================
    pub fn edit_budget(&self, new_budget_name: String, old_budget_name: String, new_total_amount: f64) -> Result<()> {
        let error_style = Style::new().red();
        let success_style = Style::new().green();

        let rows_affected = self.conn.execute(
            "UPDATE budgets SET budget_name = ?1, total_amount = ?2 WHERE budget_name = ?3",
            params![new_budget_name, new_total_amount, old_budget_name],
        )?;

        if rows_affected > 0 {
            println!("{}", success_style.apply_to("Budget modifiée avec succès !"));
        } else {
            println!("{}", error_style.apply_to(format!("Le budget '{}'n'existe pas.", old_budget_name)));
        }

        Ok(())
    }
//==================================Function to display budgets====================================
    pub fn display_budgets(&self) ->Result<()> {
        let mut stmt = self.conn.prepare("SELECT id, budget_name, total_amount, remaining_amount FROM budgets")?; // Utilisation de "?" pour extraire Statement
        let budget_iterator = stmt.query_map([], |row| {
    Ok((
        row.get::<_, i32>(0)?,      // id
        row.get::<_, String>(1)?,  // budget_name
        row.get::<_, f64>(2)?,     // total_amount
        row.get::<_, f64>(3)?,     // remaining_amount
    ))
    })?;
    let mut table = Table::new();
    println!("{}", "Liste des budgets :".bold().underline().green());
    println!("{}", "Création en cours ...".green().bold());
    table.add_row(row!["ID".bold(),"Nom du Budget".bold(),"Total $".bold(),"Restant $".bold()]);
    
    for budget in budget_iterator {
        let (id,name,total,remaining)=budget?;
        table.add_row(row![id,name,total,remaining]);

    }
    table.printstd();
    Ok(())
    }

    //==================================function to get budgets'id ===================================
    pub fn get_budget_id(&self, budget_name: &str) -> Result<Option<i32>> {  //<i64>  
        let mut statm=self.conn.prepare("SELECT id FROM budgets WHERE budget_name = ?1")?;
        

        let mut rows =statm.query(params![budget_name])?;
    

        if let Some(row) =rows.next()? {
            let id: i32 =row.get(0)?;
            Ok(Some(id))
        } else {
            Ok(None) // si aucun budget n'est trouvé
        }

    }
    pub fn afficher_alerte(message: &str) {
        println!("{}", "ALERTE:".red().bold());
        println!("{}", message.yellow().bold());
    }

//==================================calculate remaining amount===================================
pub fn calculate_remaining_amount(&self, budget_name: &str, total_amount: f64) -> Result<f64> {
    let error_style = Style::new().red();
    let success_style = Style::new().green();

    // Obtenir l'ID du budget correspondant
    let budget_id = self.get_budget_id(budget_name)?;

    if budget_id.is_none() {
        println!("{}", error_style.apply_to("Erreur : Le budget spécifié n'existe pas."));
        return Err(rusqlite::Error::InvalidParameterName("Budget non trouvé".into()));
    }
    let budget_id = budget_id.unwrap();

    // Calculer le montant total des transactions associées au budget
    let mut stmt = self.conn.prepare("SELECT COALESCE(SUM(amount), 0) FROM transactions WHERE budget_id = ?")?;
    let total_spent: f64 = stmt.query_row([budget_id], |row| {
        Ok(row.get::<usize, Option<f64>>(0)?.unwrap_or(0.0)) // Si NULL, retourner 0.0
    })?;

    let remaining_amount = total_amount - total_spent;

    if remaining_amount < total_amount * 0.1 {
        Self::afficher_alerte(&format!(
            "Le montant restant est inférieur à 10 % du budget total ({}$ restant).",
            remaining_amount
        ));
    }

    Ok(remaining_amount)
}

    /////////////////////////////////
    
    pub fn add_transaction(&self, budget_name: &str, transaction_name: String, total_amount: f64) -> Result<()> {
        let error_style = Style::new().red();
        let success_style = Style::new().green();

        let budget_id = self.get_budget_id(budget_name)?.ok_or_else(|| {
            println!("{}", error_style.apply_to(format!("Erreur : Le budget '{}' n'existe pas.", budget_name)));
            rusqlite::Error::InvalidParameterName("Budget introuvable".into())
        })?;
        

        self.conn.execute(
            "INSERT INTO transactions (budget_id, transaction_name, amount) VALUES (?1, ?2, ?3)",
            params![budget_id, transaction_name, total_amount],
        )?;

        println!("{}", success_style.apply_to("Transaction ajoutée avec succès !"));
        Ok(())
    }

    

    pub fn remove_transaction(&self, budget_name: &str, transaction_name: &str) -> Result<()> {
        let error_style = Style::new().red();
        let success_style = Style::new().green();
        let warning_style = Style::new().yellow();

        let budget_id = match self.get_budget_id(budget_name) {
            Ok(id) => id,
            Err(_) => {
                println!("{}", error_style.apply_to(format!("Erreur : Le budget '{}' n'existe pas.", budget_name)));
                return Ok(());
            }
        };

        let rows_affected = self.conn.execute(
            "DELETE FROM transactions WHERE budget_id = ?1 AND transaction_name = ?2",
            params![budget_id, transaction_name],
        )?;

        if rows_affected > 0 {
            println!("{}", success_style.apply_to(format!("Transaction '{}' supprimée avec succès dans le budget '{}'.", transaction_name, budget_name)));
        } else {
            println!("{}", warning_style.apply_to(format!("Aucune transaction correspondant à '{}' trouvée dans le budget '{}'.", transaction_name, budget_name)));
        }

        Ok(())
    }

    pub fn edit_transaction(&self, budget_name: &str, old_name: String, new_name: String, new_amount: f64) -> Result<()> {
        let error_style = Style::new().red();
        let success_style = Style::new().green();
        let warning_style = Style::new().yellow();

        let budget_id = match self.get_budget_id(budget_name) {
            Ok(id) => id,
            Err(_) => {
                println!("{}", error_style.apply_to(format!("Erreur : Le budget '{}' n'existe pas.", budget_name)));
                return Ok(());
            }
        };

        let rows_affected = self.conn.execute(
            "UPDATE transactions SET transaction_name = ?1, amount = ?2 WHERE budget_id = ?3 AND transaction_name = ?4",
            params![new_name, new_amount, budget_id, old_name],
        )?;

        if rows_affected > 0 {
            println!("{}", success_style.apply_to("Transaction modifiée avec succès !"));
        } else {
            println!("{}", warning_style.apply_to(format!("Aucune transaction correspondant à '{}' trouvée dans le budget '{}'.", old_name, budget_name)));
        }

        Ok(())
    }

    pub fn show_remaining_amount(&self, budget_name: &str) -> Result<()> {
        let success_style = Style::new().green();
        let warning_style = Style::new().red();
        let title_style = Style::new().blue().bold();
    
        // Log pour vérifier le nom du budget
        println!("Recherche du budget : {}", budget_name);
    
        let budget = match self.conn.query_row(
            "SELECT id, total_amount FROM budgets WHERE budget_name = ?1",
            params![budget_name],
            |row| Ok((row.get::<_, f64>(0)?, row.get::<_, f64>(1)?)),
        ) {
            Ok(budget) => {
                println!("Budget trouvé : ID = {}, Montant total = {}", budget.0, budget.1);
                budget
            },
            Err(_) => {
                println!("{}", warning_style.apply_to(format!("Erreur : Le budget '{}' n'existe pas.", budget_name)));
                return Ok(()); 
            }
        };
    
        let total_spent: f64 = match self
            .conn
            .query_row(
                "SELECT COALESCE(SUM(amount), 0) FROM transactions WHERE budget_id = ?1",
                params![budget.0],
                |row| row.get(0),
            ) {
                Ok(value) => value,
                Err(_) => 0.0, // Default to 0 if an error occurs
            };
    
        let remaining_amount = budget.1 - total_spent;
        let amount_style = if remaining_amount >= 0.0 {
            success_style
        } else {
            warning_style
        };
    
        println!(
            "{} : {}",
            title_style.apply_to(format!("Solde restant pour '{}' :", budget_name)),
            amount_style.apply_to(remaining_amount)
        );
       
        Ok(())
    }
}
    





    

   


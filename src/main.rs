mod db;
mod menu;

use db::BudgetManager;
use menu::interactive_menu;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let db_name = "budget_manager.db";

    // Initialiser le gestionnaire de budget
    let budget_manager = BudgetManager::new(db_name)?;

    // Lancer le menu interactif
    interactive_menu(&budget_manager)?;

    Ok(())
}

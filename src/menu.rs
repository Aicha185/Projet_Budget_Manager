use console::{Style, Term};
use std::io::{self, Write};
use crate::db::BudgetManager;

pub fn interactive_menu(budget_manager: &BudgetManager) -> Result<(), Box<dyn std::error::Error>> {
    let _term = Term::stdout();
    let prompt_style = Style::new().bold().green();
    let error_style = Style::new().red();
    let menu_style = Style::new().cyan().bold();

    loop {
        println!("{}", menu_style.apply_to("=== Menu Gestionnaire Budget ==="));
        println!("1.Ajouter un budget");
        println!("2.Supprimer un budget");
        println!("3.Modifier un budget");
        println!("4.Afficher tous les budgets");
        println!("5.Ajouter une transaction");
        println!("6.Supprimer une transaction");
        println!("7.Modifier une transaction");
        println!("8.Afficher le solde restant d'un budget");
        println!("9.Quitter\n");
        print!("{}", prompt_style.apply_to("Choisissez une option : "));
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Nom du budget : ");
                io::stdout().flush()?;
                let mut budget_name = String::new();
                io::stdin().read_line(&mut budget_name)?;
                let budget_name = budget_name.trim();

                print!("Montant total du budget : ");
                io::stdout().flush()?;
                let mut total_amount = String::new();
                io::stdin().read_line(&mut total_amount)?;

                 // Conversion du montant total
                let total_amount: f64 = match total_amount.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{}", error_style.apply_to(" Montant invalide. Réessayez "));
                        continue;
                    }
                };

                // Ajout du budget dans la base de données
             match budget_manager.add_budget(budget_name, total_amount as f64) {
             Ok(_) => {
                println!("{}", prompt_style.apply_to(format!("Le budget '{}' a été ajouté avec succès !", budget_name)));
            }
             Err(err) => {
            eprintln!("{}: {}", error_style.apply_to("Erreur lors de l'ajout du budget"), err);
        }
    }
            }
            "2" => {
                print!("Nom du budget à supprimer :");
                io::stdout().flush()?;
                let mut budget_name = String::new();
                io::stdin().read_line(&mut budget_name)?;
                let budget_name = budget_name.trim();

                if let Err(err) = budget_manager.remove_budget(budget_name) {
                    eprintln!(" Erreur : {} ", err);
                }
            }
            "3" => {
                print!("Nom du budget à modifier : ");
                io::stdout().flush()?;
                let mut old_budget_name = String::new();
                io::stdin().read_line(&mut old_budget_name)?;
                let old_budget_name = old_budget_name.trim();

                print!("Nouveau nom du budget : ");
                io::stdout().flush()?;
                let mut new_budget_name = String::new();
                io::stdin().read_line(&mut new_budget_name)?;
                let new_budget_name = new_budget_name.trim();

                print!("Nouveau montant total du budget : ");
                io::stdout().flush()?;
                let mut new_total_amount = String::new();
                io::stdin().read_line(&mut new_total_amount)?;
                let new_total_amount: f64 = match new_total_amount.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{}", error_style.apply_to(" Montant invalide. Réessayez  "));
                        continue;
                    }
                };

                if let Err(err) = budget_manager.edit_budget(new_budget_name.to_string(), old_budget_name.to_string(), new_total_amount) {
                    eprintln!(" Erreur : {}", err);
                }
            }
            "4" => {
                if let Err(err) = budget_manager.display_budgets() {
                    eprintln!(" Erreur : {}", err);
                }
            }
            "5" => {
                print!("Nom du budget : ");
                io::stdout().flush()?;
                let mut budget_name = String::new();
                io::stdin().read_line(&mut budget_name)?;
                let budget_name = budget_name.trim();

                print!("Nom de la transaction : ");
                io::stdout().flush()?;
                let mut transaction_name = String::new();
                io::stdin().read_line(&mut transaction_name)?;
                let transaction_name = transaction_name.trim();

                print!("Montant de la transaction :\n ");
                io::stdout().flush()?;
                let mut amount = String::new();
                io::stdin().read_line(&mut amount)?;
                let amount: f64 = match amount.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{}", error_style.apply_to("Montant invalide. Réessayez "));
                        continue;
                    }
                };

                if let Err(err) = budget_manager.add_transaction(budget_name, transaction_name.to_string(), amount) {
                    eprintln!(" Erreur : {}", err);
                }
            }
            "6" => {
                print!("Nom du budget : ");
                io::stdout().flush()?;
                let mut budget_name = String::new();
                io::stdin().read_line(&mut budget_name)?;
                let budget_name = budget_name.trim();

                print!("Nom de la transaction à supprimer : ");
                io::stdout().flush()?;
                let mut transaction_name = String::new();
                io::stdin().read_line(&mut transaction_name)?;
                let transaction_name = transaction_name.trim();

                if let Err(err) = budget_manager.remove_transaction(budget_name, transaction_name) {
                    eprintln!("Erreur : {}", err);
                }
            }
            "7" => {
                print!("Nom du budget : ");
                io::stdout().flush()?;
                let mut budget_name = String::new();
                io::stdin().read_line(&mut budget_name)?;
                let budget_name = budget_name.trim();

                print!("Nom de la transaction à modifier : ");
                io::stdout().flush()?;
                let mut old_name = String::new();
                io::stdin().read_line(&mut old_name)?;
                let old_name = old_name.trim();

                print!("Nouveau nom de la transaction : ");
                io::stdout().flush()?;
                let mut new_name = String::new();
                io::stdin().read_line(&mut new_name)?;
                let new_name = new_name.trim();

                print!("Nouveau montant de la transaction : ");
                io::stdout().flush()?;
                let mut new_amount = String::new();
                io::stdin().read_line(&mut new_amount)?;
                let new_amount: f64 = match new_amount.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("{}", error_style.apply_to("Montant invalide. Réessayez "));
                        continue;
                    }
                };

                if let Err(err) = budget_manager.edit_transaction(budget_name, old_name.to_string(), new_name.to_string(), new_amount) {
                    eprintln!(" Erreur : {}", err);
                }
            }
            "8" => {
                print!("Nom du budget : ");
                io::stdout().flush()?;
                let mut budget_name = String::new();
                io::stdin().read_line(&mut budget_name)?;
                let budget_name = budget_name.trim();
                
               //
                if let Err(err) = budget_manager.show_remaining_amount(budget_name) {
                    eprintln!("Erreur : {}", err);
                }
            }
            "9" => {
                println!("{}", menu_style.apply_to("Au revoir !!!"));
                break;
            }
            _ => {
                println!("{}", error_style.apply_to("Option invalide, veuillez réessayer "));
            }
        }

        println!(); // Ligne vide pour la lisibilité
    }

    Ok(())
}

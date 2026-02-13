use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use chrono::Local;
use std::fs;

const FILE: &str = "expenses.json";

#[derive(Parser)]
#[command(name = "pocket")]
#[command(about = "Simple Expense Tracker CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]


enum Commands {
    Add {
        amount: f64,
        desc: String,
        cat: String,
    },
    List,
    Total,
    Summary,
    Export,
}

#[derive(Serialize, Deserialize)]
struct Expense {
    amount: f64,
    desc: String,
    cat: String,
    date: String,
}

fn load_expenses() -> Vec<Expense> {
    if let Ok(data) = fs::read_to_string(FILE) {
        serde_json::from_str(&data).unwrap_or(vec![])
    } else {
        vec![]
    }
}

fn save_expenses(data: &Vec<Expense>) {
    fs::write(FILE, serde_json::to_string_pretty(data).unwrap()).unwrap();
}

fn main() {
    let cli = Cli::parse();
    let mut expenses = load_expenses();

    match cli.command {
        Commands::Add { amount, desc, cat } => {
            let e = Expense {
                amount,
                desc,
                cat,
                date: Local::now().format("%Y-%m-%d").to_string(),
            };
            expenses.push(e);
            save_expenses(&expenses);
            println!("Expense saved.");
        }

        Commands::List => {
            for e in expenses {
                println!("{} | {}€ | {} | {}", e.date, e.amount, e.cat, e.desc);
            }
        }

        Commands::Total => {
            let total: f64 = expenses.iter().map(|e| e.amount).sum();
            println!("Total spent: {:.2}€", total);
        }

        Commands::Summary => {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            for e in expenses {
                *map.entry(e.cat).or_insert(0.0) += e.amount;
            }
            for (cat, sum) in map {
                println!("{}: {:.2}€", cat, sum);
            }
        }

        Commands::Export => {
            let mut csv = "date,amount,category,description\n".to_string();
            for e in expenses {
                csv.push_str(&format!("{},{},{},{}\n", e.date, e.amount, e.cat, e.desc));
            }
            fs::write("expenses.csv", csv).unwrap();
            println!("Exported to expenses.csv");
        }
    }
}

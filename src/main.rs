use std::env;
use invidivual_project_rust_yabei::{extract, load, query, update_price};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            let file_path = extract(
                Some("https://raw.githubusercontent.com/yabeizeng1121/Mini_Project5/main/cars.csv"),
                Some("cars.csv"),
            );
            println!("Data extraction completed successfully. Saved to {}", file_path);
        }
        "load" => {
            let db_path = load(Some("cars.csv"));
            println!("Data transformation and loading completed successfully. DB path: {}", db_path);
        }
        "query" => {
            query();
            println!("Data querying completed successfully.");
        }
        "update_price" => {
            if args.len() < 4 {
                println!("Usage: {} update_price [brand] [new_price]", args[0]);
                return;
            }
            let brand = &args[2];
            let new_price: f64 = args[3].parse().unwrap_or(0.0);
            match update_price(brand, new_price) {
                Ok(_) => println!("Updated price for brand {} to {}", brand, new_price),
                Err(e) => println!("Error updating price: {}", e),
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'load', 'query', or 'update_price'.");
        }
    }
}

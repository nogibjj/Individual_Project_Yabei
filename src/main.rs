use invidivual_project_rust_yabei::{extract, transform_load, query};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            let result = extract(
                "https://raw.githubusercontent.com/yabeizeng1121/Mini_Project5/main/cars.csv",
                "cars.csv",
            );
            match result {
                Ok(path) => println!("Data extraction completed successfully. Saved to {}", path),
                Err(e) => eprintln!("Error during extraction: {:?}", e),
            }
        }
        "transform_load" => {
            let result = transform_load("cars.csv");
            match result {
                Ok(path) => println!(
                    "Data transformation and loading completed successfully. DB path: {}",
                    path
                ),
                Err(e) => eprintln!("Error during loading: {:?}", e),
            }
        }
        "query" => {
            if args.len() < 3 {
                println!("Usage: {} query [SQL query string]", args[0]);
                return;
            }
            let query_string = &args[2];
            let results = query();
            match results {
                Ok(rows) => {
                    println!("Top 5 rows of the CarsDB table:");
                    for row in rows {
                        println!("{:?}", row);
                    }
                }
                Err(e) => eprintln!("Error executing query: {:?}", e),
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
        }
    }
}

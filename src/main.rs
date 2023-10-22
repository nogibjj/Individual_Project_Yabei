use invidivual_project_rust_yabei::{extract, load, query};
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
        _ => {
            println!("Invalid action. Use 'extract', 'load', or 'query'.");
        }
    }
}

mod cars_lib;

fn main() {
    // Extract
    println!("Extracting data...");
    match cars_lib::extract(
        "https://raw.githubusercontent.com/yabeizeng1121/Mini_Project5/main/cars.csv",
        "cars.csv",
    ) {
        Ok(file_path) => {
            println!("Data extraction completed successfully. Saved to {}\n", file_path);

            // Transform and Load
            println!("Transforming and loading data...");
            match cars_lib::load(&file_path) {
                Ok(_) => {
                    println!("Data transformation and loading completed successfully.\n");

                    // Query
                    println!("Querying data...");
                    match cars_lib::query() {
                        Ok(_) => println!("Data querying completed successfully.\n"),
                        Err(e) => eprintln!("An error occurred while querying data: {}", e),
                    }
                }
                Err(e) => eprintln!("An error occurred while transforming and loading data: {}", e),
            }
        }
        Err(e) => eprintln!("An error occurred while extracting data: {}", e),
    }
}

use invidivual_project_rust_yabei::{extract, transform_load, query};

#[test]
fn test_extract() {
    let url = "https://raw.githubusercontent.com/yabeizeng1121/Mini_Project5/main/cars.csv";
    let file_path = "cars_test.csv";

    extract(url, file_path).unwrap();

    // Check if the file was successfully created
    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "cars_test.csv";  // Using the file created by test_extract
    let result = transform_load(dataset);

    // Check if the database was successfully created
    assert_eq!(result.unwrap(), "CarsDB.db");
}

#[test]
fn test_query() {
    // Query the top 5 rows from the CarsDB table
    let result = query();

    // Check if the query was successful and returns 5 rows
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 5);
}

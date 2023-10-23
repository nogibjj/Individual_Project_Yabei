use csv::ReaderBuilder;
use reqwest;
use rusqlite::{params, Connection};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::Write;
use std::fs::OpenOptions;
use std::fs;
use reqwest::blocking::Client;

const LOG_FILE: &str = "query_log.md";
#[derive(Debug)]


fn log_query(query: &str, log_file: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(log_file)
        .unwrap();
    writeln!(file, "{}", query).unwrap();
}

pub struct Car {
    _car: String,
    _mpg: f64,
    _cylinders: i32,
    _displacement: f64,
    _horsepower: f64,
    _weight: f64,
    _acceleration: f64,
    _model: i32,
    _origin: String,
}

// Function to extract data from a URL and save to a file
pub fn extract(url: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let mut file = File::create(file_path)?;
    file.write_all(&content)?;
    Ok(file_path.to_string())
}

// Function to load CSV data into SQLite
pub fn transform_load(dataset: &str) -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::open("CarsDB.db")?;

    conn.execute("DROP TABLE IF EXISTS CarsDB", params![])?;

    let create_table_query = "
        CREATE TABLE CarsDB (
            Car TEXT,
            MPG REAL,
            Cylinders INTEGER,
            Displacement REAL,
            Horsepower REAL,
            Weight REAL,
            Acceleration REAL,
            Model INTEGER,
            Origin TEXT
        )
    ";
    conn.execute(create_table_query, params![])?;

    let file = File::open(dataset)?;
    let reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(reader);

    for result in rdr.records() {
        let record = result?;
        conn.execute(
            "INSERT INTO CarsDB (Car, MPG, Cylinders, Displacement, Horsepower, Weight, Acceleration, Model, Origin) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                record.get(0).unwrap_or(""),
                record.get(1).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(2).unwrap_or("0").parse::<i32>().unwrap_or(0),
                record.get(3).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(4).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(5).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(6).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0),
                record.get(7).unwrap_or("0").parse::<i32>().unwrap_or(0),
                record.get(8).unwrap_or("")
            ],
        )?;
    }

    Ok("CarsDB.db".to_string())
}

pub fn query(query_string: &str) -> Result<String, rusqlite::Error> {
    let conn = Connection::open("CarsDB.db")?;

    if query_string
        .trim_start()
        .to_uppercase()
        .starts_with("SELECT")
    {
        let mut stmt = conn.prepare(query_string)?;
        let car_iter = stmt.query_map([], |row| {
            Ok(Car {
                _car: row.get(0)?,
                _mpg: row.get(1)?,
                _cylinders: row.get(2)?,
                _displacement: row.get(3)?,
                _horsepower: row.get(4)?,
                _weight: row.get(5)?,
                _acceleration: row.get(6)?,
                _model: row.get(7)?,
                _origin: row.get(8)?,
            })
        })?;

        for result in car_iter {
            match result {
                Ok(car) => {
                    println!(
                        "Results: Car = {}, MPG = {}, Cylinders = {}, Displacement = {}, Horsepower = {}, Weight = {}, Acceleration = {}, Model = {}, Origin = {}",
                        car._car, car._mpg, car._cylinders, car._displacement, car._horsepower, car._weight, car._acceleration, car._model, car._origin
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        let _num_affected_rows = conn.execute_batch(query_string)?;
    }
    log_query(query_string, LOG_FILE);
    Ok("Query executed successfully".to_string())
}








                   
                    
 

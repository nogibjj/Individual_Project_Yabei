use reqwest;
use rusqlite::{params, Connection};
use std::fs::File;
use std::io::{BufReader, prelude::*};
use csv::ReaderBuilder;

#[derive(Debug)]
pub struct Car {
    car: String,
    mpg: f64,
    cylinders: i32,
    displacement: f64,
    horsepower: f64,
    weight: f64,
    acceleration: f64,
    model: i32,
    origin: String,
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
    
    if query_string.trim_start().to_uppercase().starts_with("SELECT") {
        let mut stmt = conn.prepare(query_string)?;
        let car_iter = stmt.query_map([], |row| {
            Ok(Car {
                car: row.get(0)?,
                mpg: row.get(1)?,
                cylinders: row.get(2)?,
                displacement: row.get(3)?,
                horsepower: row.get(4)?,
                weight: row.get(5)?,
                acceleration: row.get(6)?,
                model: row.get(7)?,
                origin: row.get(8)?,
            })
        })?;
        
        for car in car_iter {
            println!("{:?}", car?);
        }
    } else {
        // For non-SELECT statements, just execute the query
        conn.execute(query_string, [])?;
    }
    
    Ok("Query executed successfully".to_string())
}

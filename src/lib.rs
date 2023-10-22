use std::fs::File;
use std::io::Write;
use reqwest;
use rusqlite::Connection;
use csv;



pub fn extract(
    url: &str,
    file_path: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    
    let mut file = File::create(file_path)?;
    file.write_all(&content)?;

    Ok(file_path.to_string())
}


pub fn query() -> Result<String, rusqlite::Error> {
    let conn = Connection::open("CarsDB.db")?;
    
    let mut stmt = conn.prepare("SELECT * FROM CarsDB LIMIT 5")?;
    let rows = stmt.query_map([], |row| {
        (
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?
        )
    })?;

    println!("Top 5 rows of the CarsDB table:");
    for row in rows {
        println!("{:?}", row?);
    }

    Ok("Success".to_string())
}


pub fn load(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    
    let mut payload: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if record.len() != 9 {
            println!("Incorrect number of fields in row: {:?}", record);
            continue;
        }
        payload.push(record.iter().map(|s| s.to_string()).collect());
    }

    let conn = Connection::open("CarsDB.db")?;
    conn.execute("DROP TABLE IF EXISTS CarsDB", [])?;

    conn.execute(
        "CREATE TABLE CarsDB (
            Brand TEXT, Price REAL, Body TEXT, Mileage INTEGER,
            EngineV REAL, Engine_Type TEXT, Registration TEXT,
            Year INTEGER, Model TEXT
        )")?;

    let tx = conn.transaction()?;
    for row in &payload {
        tx.execute(
            "INSERT INTO CarsDB (Brand, Price, Body, Mileage, EngineV, Engine_Type, 
            Registration, Year, Model) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &row,
        )?;
    }
    tx.commit()?;

    Ok("CarsDB.db".to_string())
}


pub fn update_price(brand: &str, new_price: f64) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("CarsDB.db")?;
    conn.execute(
        "UPDATE CarsDB SET Price = ?1 WHERE Brand = ?2",
        &[new_price, brand],
    )?;
    Ok(())
}




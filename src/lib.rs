use std::fs::File;
use std::io::Write;
use reqwest;
use rusqlite::{Connection, ToSql}; 
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

pub fn query(query_string: &str) -> Result<String, rusqlite::Error> {
    let conn = Connection::open("CarsDB.db")?;
    
    match conn.execute(query_string, []) {
        Ok(_) => return Ok("Query executed successfully.".to_string()),
        Err(_) => {}  
    }

    let mut stmt = conn.prepare(query_string)?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i32>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, i32>(7)?,
            row.get::<_, String>(8)?
        ))
    })?;

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

    let mut conn = Connection::open("CarsDB.db")?;
    conn.execute("DROP TABLE IF EXISTS CarsDB", [])?;

    conn.execute(
        "CREATE TABLE CarsDB (
            Brand TEXT, Price REAL, Body TEXT, Mileage INTEGER,
            EngineV REAL, Engine_Type TEXT, Registration TEXT,
            Year INTEGER, Model TEXT
        )", [])?;

    let tx = conn.transaction()?;
    for row in &payload {
        let params: Vec<&dyn ToSql> = row.iter().map(|s| s as &dyn ToSql).collect();
        tx.execute(
            "INSERT INTO CarsDB (Brand, Price, Body, Mileage, EngineV, Engine_Type, 
            Registration, Year, Model) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &params[..],  // Convert Vec to slice
        )?;
    }
    tx.commit()?;

    Ok("CarsDB.db".to_string())
}

pub fn update_price(brand: &str, new_price: f64) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("CarsDB.db")?;
    conn.execute(
        "UPDATE CarsDB SET Price = ?1 WHERE Brand = ?2",
        &[&new_price as &dyn ToSql, &brand as &dyn ToSql],
    )?;
    Ok(())
}

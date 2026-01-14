//use serde_json::{Value, Result};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
//use std::io::ErrorKind;
use std::io::BufWriter;
//use serde::Serialize;
use std::path::Path;

use serde_json::Result as JsonResult;
use std::result::Result;

pub fn import_configuration_file(path: &str) -> JsonResult<Value> {
    // 1. Apri il file
    let file = File::open(path).expect("Unable to open file");
    
    // 2. Crea un lettore con buffer (più efficiente)
    let reader = BufReader::new(file);

    // 3. Passa il lettore direttamente a serde_json
    let v: Value = serde_json::from_reader(reader)?;

    Ok(v)
}


pub fn write_condition_file<P: AsRef<Path>>(
    path: P, 
    points: &[crate::structs::Point] //Importato correttamente
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create/Open the file
    let file = File::create(path)?;
    
    // 2. Wrap in a BufWriter to minimize system calls
    let writer = BufWriter::new(file);

    // 3. Serialize directly to the writer using bincode
    // This creates a compact binary representation
    bincode::serialize_into(writer, points)?;

    Ok(())
}


pub fn  read_condition_file<P: AsRef<Path>>(path: P) -> Result<Vec<super::structs::Point>, Box<dyn std::error::Error>> 
{ 
    //use std::fs::File;
    //use std::io::BufReader;
    //use bincode;
    
    // 1. Open the file
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    // 2. Deserialize the data from the file
    let points: Vec<crate::structs::Point> = bincode::deserialize_from(reader)?;
    
    Ok(points)
} 

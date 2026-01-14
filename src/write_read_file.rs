use serde_json::{Value, Result};
use std::fs::File;
use std::io::BufReader;
//use std::io::ErrorKind;
use std::io::BufWriter;
use serde::Serialize;
use std::path::Path;



pub fn import_configuration_file(path: &str) -> Result<Value> {
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
    points: &[super::structs::Point]
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

/*
pub fn write_condition_file<P: AsRef<Path>,T: Serialize>(path: P, dati: &[T]) -> ()//Result<(), Box<dyn std::error::Error>> 
{
    // 1. Opening file in write mode
    let file = File::create(path); //?;
    
    // 2. Using a BufWriter to optimize writing
    let mut writer = BufWriter::new(file);

    // 3. Serialization of data directly into file
    bincode::serialize_into(&mut writer, dati); //?;

    // 4. Ensure all the data are correctly written in the file
    Ok(())
}
*/

/*
pub fn write_condition_file(path: &str, data: &Vec<super::structs::Point>) -> () {
    use std::io::Write;
    use bincode;
    
    // 1. Create the file
    let mut file = File::create(path)?;
    
    // 2. Serialize the data and write it to the file
    let encoded: Vec<u8> = bincode::serialize(data)?;
    file.write_all(&encoded)?;
    
    Ok(())
}
*/

/*
pub fn write_condition_file(path: &str, data: &Vec<super::structs::Point>) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    use bincode;
    
    // 1. Create the file
    let mut file = File::create(path)?;
    
    // 2. Serialize the data and write it to the file
    let encoded: Vec<u8> = bincode::serialize(data)?;
    file.write_all(&encoded)?;
    
    Ok(())
}

pub fn read_condition_file(path: &str) -> std::io::Result<Vec<super::structs::Point>> { 
    use std::fs::File;
    use std::io::BufReader;
    use bincode;
    
    // 1. Open the file
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    // 2. Deserialize the data from the file
    let points: Vec<super::structs::Point> = bincode::deserialize_from(reader)?;
    
    Ok(points)
} 
*/
use serde_json::{Value, Result};
use std::fs::File;
use std::io::BufReader;

pub fn import_configuration_file(path: &str) -> Result<Value> {
    // 1. Apri il file
    let file = File::open(path).expect("Unable to open file");
    
    // 2. Crea un lettore con buffer (più efficiente)
    let reader = BufReader::new(file);

    // 3. Passa il lettore direttamente a serde_json
    let v: Value = serde_json::from_reader(reader)?;

    Ok(v)
}






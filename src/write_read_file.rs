use serde_json::{Value, Result};
use std::fs::File;
use std::io::BufReader;

use magnetic_monopoles::structs;

pub fn import_configuration_file(path: &str) -> Result<Value> {
    // 1. Apri il file
    let file = File::open(path).expect("Unable to open file");
    
    // 2. Crea un lettore con buffer (più efficiente)
    let reader = BufReader::new(file);

    // 3. Passa il lettore direttamente a serde_json
    let v: Value = serde_json::from_reader(reader)?;

    Ok(v)
}

/*
pub fn write_initial_condition_file(path: &str, data: &Vec<super::structs::Point>) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = File::create(path)?;

    for point in data {
        
    }

    Ok(())
}


fn main() -> bincode::Result<()> {
    // 1. Dati di esempio
    let utenti = vec![
        Utente { nome: "Alice".to_string(), id: 1, attivo: true },
        Utente { nome: "Bob".to_string(), id: 2, attivo: false },
    ];

    // 2. Crea il file (usiamo BufWriter per migliorare le performance di scrittura)
    let file = File::create("utenti.bin").expect("Impossibile creare il file");
    let mut writer = BufWriter::new(file);

    // 3. Serializza direttamente nel file
    bincode::serialize_into(&mut writer, &utenti)?;
    
    println!("Array di struct salvato in formato binario!");
    Ok(())
}

*/
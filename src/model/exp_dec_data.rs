use std::error::Error;
use std::path;
use std::fs;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RecordDec {
    time: i32,
    total_n_dec: i32,
}

impl RecordDec {
    pub fn new(time: i32, total_n_dec: i32) -> RecordDec {
        RecordDec { time, total_n_dec }
    }
}

pub fn write_results(records: Vec<RecordDec>,  folder: &path::PathBuf) -> Result<(), Box<dyn Error>> {
    let file = folder.join("results.csv");
    //let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut wtr = csv::Writer::from_path(file)?;

    // When writing records with Serde using structs, the header row is written
    // automatically.
    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn _copy_config(folder: &path::PathBuf) -> std::io::Result<()>{
    let file = folder.join("model_config.toml");
    fs::copy("model_config.toml", file)?;  // Copy foo.txt to bar.txt
    Ok(())
}
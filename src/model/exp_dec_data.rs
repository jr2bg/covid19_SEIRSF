use std::error::Error;
use std::io;

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

pub fn write_results(records: Vec<RecordDec>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // When writing records with Serde using structs, the header row is written
    // automatically.
    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}

use std::error::Error;
use std::io;
use std::process;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Record_Dec {
    time : i32,
    total_n_dec : i32,
}

impl Record_Dec {
    pub fn new(time: i32, total_n_dec: i32) -> Record_Dec {
        Record_Dec { time, total_n_dec }
    }
}

pub fn write_results(records : Vec<Record_Dec>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // When writing records with Serde using structs, the header row is written
    // automatically.
    for record in records {
        wtr.serialize(record)?;
    }
    /*wtr.serialize(Record {
        city: "Southborough".to_string(),
        region: "MA".to_string(),
        country: "United States".to_string(),
        population: Some(9686),
    })?;
    wtr.serialize(Record {
        city: "Northbridge".to_string(),
        region: "MA".to_string(),
        country: "United States".to_string(),
        population: Some(14061),
    })?;*/
    wtr.flush()?;
    Ok(())
}
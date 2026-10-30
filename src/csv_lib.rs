use std::error::Error;
use std::io;
use std::path::PathBuf;
use csv::StringRecord;
use serde::Deserialize;

//#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CsvRecord {

    #[serde(skip_deserializing)]
    pub dub_id: String,

    pub story_id: String,
    pub choice_id: String,
    pub hashcode: String,
    pub filename: String,
    pub character: String,
    pub characters: String,
    pub button_text: String,
    pub sets: String,
    pub line_to_dub: String,
    pub requirements: String,
    pub path: String,
    pub level: String,
    pub is_dream: String,
    pub is_flirt: String,
    pub is_groundhog: String,
    pub has_battle: String,
}



pub fn parse_csv_records() -> Result<Vec<CsvRecord>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("AllLinesDump.csv")?;
    let mut csv_vec: Vec<CsvRecord> = Vec::new();
    for result in rdr.deserialize() {
        let mut record: CsvRecord = result?;

        record.dub_id = format!("{}_{}", record.story_id, record.hashcode);
        //println!("{:?}", record);
        csv_vec.push(record);
        // Try this if you don't like each record smashed on one line:
        // println!("{:#?}", record);
    }
    Ok(csv_vec)
}







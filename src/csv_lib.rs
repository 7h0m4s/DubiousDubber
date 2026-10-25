use std::error::Error;
use std::io;
use std::path::PathBuf;
use csv::StringRecord;
use serde::Deserialize;

//#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvRecord {
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
        let record: CsvRecord = result?;
        //println!("{:?}", record);
        csv_vec.push(record);
        // Try this if you don't like each record smashed on one line:
        // println!("{:#?}", record);
    }
    Ok(csv_vec)
}





// pub(crate) struct DubCsvContent {
//     headers: Vec<String>,
//     rows: Vec<Vec<String>>,
//     file_path: Option<PathBuf>,
//     striped: bool,
//     resizable: bool,
// }
// impl DubCsvContent {
//     pub fn new(path: PathBuf) -> Self {
//         match std::fs::File::open(path) {
//             Ok(file) => {
//                 let mut csv_reader = csv::ReaderBuilder::new()
//                     .has_headers(true)
//                     .from_reader(file);
//
//                 let headers: Vec<String> = csv_reader
//                     .headers()
//                     .map(|h| h.iter().map(|s| s.to_string()).collect())
//                     .unwrap_or_default();
//
//
//                 let mut saved_rows: Vec<Vec<String>> = Vec::new();
//                 for csv_row in csv_reader.records() {
//                     let csv_some_row = Some(csv_row).expect("Failed to get CSV row");
// let csv_row_iter = csv_some_row.Iter();
//
//                     let mut new_saved_row: Vec<String> = Vec::new();
//
//                     for column in csv_some_row.iter(){
//                         new_saved_row.push(column);
//                     }
//                     saved_rows.push(new_saved_row);
//                 }
//
//
//                 Self {
//                     headers: headers,
//                     rows: saved_rows,
//                     file_path: Some(path.clone()),
//                     striped: false,
//                     resizable: false,
//                 }
//
//             }
//             Err(e) => {
//                 eprintln!("Failed to open CSV: {e}");
//                 Self{
//                     headers: vec![],
//                     rows: vec![],
//                     file_path: None,
//                     striped: false,
//                     resizable: false,
//                 }
//             }
//
//         }
//
//     }
// }

// self.rows = csv_reader
//     .records()
//     .map(|r| r.iter().map(|s| s.Unwrap().to_string()).collect())
//     .collect();

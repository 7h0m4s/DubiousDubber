use std::error::Error;
use std::io;
use std::path::PathBuf;
use csv::StringRecord;
use serde::Deserialize;

//#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CsvRecord {
    story_id: String,
    choice_id: String,
    hashcode: String,
    filename: String,
    character: String,
    characters: String,
    button_text: String,
    sets: String,
    line_to_dub: String,
    requirements: String,
    path: String,
    level: String,
    is_dream: String,
    is_flirt: String,
    is_groundhog: String,
    has_battle: String,
}


pub fn parse_csv_records() -> Result<Vec<CsvRecord>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("AllLinesDump.csv")?;
    let mut csv_vec: Vec<CsvRecord> = Vec::new();
    for result in rdr.deserialize() {
        let record: CsvRecord = result?;
        println!("{:?}", record);
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

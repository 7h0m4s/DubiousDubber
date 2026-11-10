use crate::{CSV_FILEPATH, DubiousDubsState};
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

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

pub fn parse_csv_records(csv_path: &PathBuf) -> Result<Vec<Rc<CsvRecord>>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    let mut csv_vec: Vec<Rc<CsvRecord>> = Vec::new();
    for result in rdr.deserialize() {
        let mut record: CsvRecord = result?;

        record.dub_id = format!("{}_{}", record.story_id, record.hashcode);
        //println!("{:?}", record);
        csv_vec.push(Rc::new(record));
        // Try this if you don't like each record smashed on one line:
        // println!("{:#?}", record);
    }

    Ok(csv_vec)
}

pub fn initialize_csv_state(state: &mut DubiousDubsState, path_buf: Option<PathBuf>) {
    state.csv_filter = String::default();
    state.filtered_csv_records = Vec::<Rc<CsvRecord>>::default();
    state.selected_csv_record = Rc::default();
    state.selected_dub_id = String::default();
    state.selected_attempt_filename = String::default();
    state.current_attempt_filenames = Vec::<String>::default();
    state.selected_attempt_filename = String::default();

    let csv_filepath: PathBuf = if (path_buf.is_none()) {
        PathBuf::from_str(CSV_FILEPATH).unwrap_or_default()
    } else {
        path_buf.unwrap_or_default()
    };
    let csv_results = parse_csv_records(&csv_filepath)
        .map_err(|error| {
            state.toasts.error(error.to_string());
        })
        .unwrap_or_default();

    if !csv_results.is_empty() {
        let rec = &csv_results[0];
        state.selected_dub_id = rec.dub_id.to_string();
        state.selected_csv_record = rec.clone();
    }
    state.csv_records = csv_results;

    state.is_first_frame = true;
    state.has_filtered_csv_changed = true;
}

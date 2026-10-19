use std::path::PathBuf;

struct DubCsvContent {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    file_path: Option<PathBuf>,
    striped: bool,
    resizable: bool,
}

impl DubCsvContent {
    fn load_csv(&mut self, path: &PathBuf) {
        match std::fs::File::open(path) {
            Ok(file) => {
                let mut rdr = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .from_reader(file);

                self.headers = rdr
                    .headers()
                    .map(|h| h.iter().map(|s| s.to_string()).collect())
                    .unwrap_or_default();

                self.rows = rdr
                    .records()
                    .filter_map(|r| r.ok())
                    .map(|r| r.iter().map(|s| s.to_string()).collect())
                    .collect();

                self.file_path = Some(path.clone());
            }
            Err(e) => {
                eprintln!("Failed to open CSV: {e}");
            }
        }
    }
}
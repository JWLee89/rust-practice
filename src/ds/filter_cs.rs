use std::fs::{File, self};
use std::io::{self, prelude::*, BufReader};

struct CsvRow {
    row: Vec<String>
}

struct CsvEntry<T>{
    key: String,
    Value: T
}

struct CsvSpec {
    headers: Vec<String>,
    
}


#[derive(Debug)]
pub(crate) struct CsvFilterer {
    file_path: String,
}


impl CsvFilterer {
    /// Creates a new [`CsvFilterer`].
    pub fn new(file_path: String) -> Self {
        CsvFilterer { file_path } 
    }

    /***
     * Read the CSV file and return a vector of rows
     * @param &self: The current instance of the struct
     * @return Vec<CsvRow>
     */
    pub fn read(&self) -> io::Result<()> {
        let mut rows: Vec<CsvRow> = Vec::new();
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line?);
        }
        Ok(())
    }

}



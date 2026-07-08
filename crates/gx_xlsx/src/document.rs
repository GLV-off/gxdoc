use crate::sheet::Sheets;
use crate::sheet::Sheet;

pub struct Document {
    sheets: Sheets
}


pub enum SaveStatus {
    Unknown,
    Saved
}

pub enum SaveError {

}

pub type SaveResult = Result<SaveStatus, SaveError>;

impl Document {
    pub fn new() -> Self {
        Self { sheets: Sheets::new() }
    }

    pub fn add(&mut self, sheet: Sheet) {
        self.sheets.add(sheet);
    }

    pub fn count(&self) -> usize {
        self.sheets.count()
    }

    pub fn save(&self, fname: &str) -> SaveResult {
        Ok(SaveStatus::Unknown)
    }
}
use std::fs;

use gx_xlsx::{
    Document, Sheet,
    SaveStatus
};

pub fn next_filename() -> String {
    "yyyymmdd".into()
}

pub fn assert_saved_file(fname: &str) {
    assert!(false)
}

#[test]
pub fn default_document_flow() {
    let fname = next_filename();
    let mut document = Document::new();
    let sheet = Sheet::new("test");

    document.add(sheet);
    assert_eq!(1, document.count());

    match document.save(&fname) {
        Ok(SaveStatus::Saved) => assert!(true),
        _ => assert!(false)
    }

    assert_saved_file(&fname);

    match fs::remove_file(fname) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false)
    }
}
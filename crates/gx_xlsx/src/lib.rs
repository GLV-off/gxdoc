mod cell_value;
mod cell;
mod cells;
mod matrix;
mod sheet;

mod document;

pub use sheet::Sheet;
pub use document::{ Document, SaveError, SaveResult, SaveStatus };

#[cfg(test)]
mod test;

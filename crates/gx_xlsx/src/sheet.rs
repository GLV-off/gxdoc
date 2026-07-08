use super::cells::Cells;

pub struct Sheet {
    short_name: String,
    cells: Cells,
}

impl Sheet {
    pub fn new(name: &str) -> Self {
        Self {
            short_name: name.into(), 
            cells: Cells::new()
        }
    }

    pub fn short_name(&self) -> String {
        self.short_name.clone()
    }
}

pub struct Sheets(Vec<Sheet>);

impl Sheets {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, sheet: Sheet) {
        self.0.push(sheet);
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}



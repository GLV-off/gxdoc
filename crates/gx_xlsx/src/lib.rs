enum CellValue {
    Undefined,
    Text, 
    Number,
}

struct Cell {
    value: CellValue
}

struct Matrix<T> {
    items: Vec<T>
}

struct Cells(Matrix<Cell>);

struct Sheet {
    short_name: String,
    cells: Cells,
}

struct Sheets(Vec<Sheet>);

struct Document {
    sheets: Sheets
}

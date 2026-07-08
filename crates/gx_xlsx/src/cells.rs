use crate::matrix::Matrix;
use crate::cell::Cell;

pub struct Cells(Matrix<Cell>);

impl Cells {
    pub fn new() -> Self {
        Self (Matrix::new())
    }
}
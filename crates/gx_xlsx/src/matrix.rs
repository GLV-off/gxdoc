pub struct Matrix<T> {
    items: Vec<T>
}

impl<T> Matrix<T> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
}
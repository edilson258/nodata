#[derive(Debug)]
pub enum DataType {
    String,
    Number,
}

#[derive(Debug)]
pub enum FieldValue {
    DataType(DataType),
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: FieldValue,
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}

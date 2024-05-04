#[derive(Debug)]
pub enum DataType {
    String,
    Number,
}

#[derive(Debug)]
pub struct SchemaField {
    pub name: String,
    pub type_: DataType,
}

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    pub fields: Vec<SchemaField>
}

#[derive(Debug)]
pub enum ModelFieldValue {
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub struct ModelField {
    pub name: String,
    pub value: ModelFieldValue,
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub fields: Vec<ModelField>,
}

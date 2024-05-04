use core::fmt;

#[derive(Debug, PartialEq)]
pub enum DataType {
    String,
    Number,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String => write!(f, "[String]"),
            Self::Number => write!(f, "[Number]"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub struct SchemaField {
    pub name: String,
    pub type_: DataType,
}

#[derive(Debug)]
pub struct Schema {
    pub name: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Debug)]
pub struct ModelField {
    pub name: String,
    pub value: Object,
    pub type_: DataType,
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub fields: Vec<ModelField>,
}

impl Model {
    pub fn find_field(&self, name: &str) -> Option<&ModelField> {
        self.fields.iter().find(|f| f.name == name)
    }
}

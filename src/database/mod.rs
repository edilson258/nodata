use crate::ast::{Model, Object, Schema, SchemaField};

#[derive(Debug)]
pub struct Row {
    id: usize,
    entries: Vec<Object>,
}

#[derive(Debug)]
pub struct Table {
    name: String,
    next_id: usize,
    colums: Vec<SchemaField>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(schema: Schema) -> Self {
        Self {
            name: schema.name,
            next_id: 1,
            colums: schema.fields,
            rows: vec![],
        }
    }

    fn get_id(&mut self) -> usize {
        let next_id = self.next_id;
        self.next_id += 1;
        next_id
    }

    pub fn get_by_id(&self, id: usize) -> Option<&Row> {
        self.rows.iter().find(|r| r.id == id)
    }

    pub fn add_row(&mut self, model: Model) -> Result<usize, String> {
        if model.fields.len() != self.colums.len() {
            return Err(format!(
                "'{}' expects {} args but found {}",
                &self.name,
                self.colums.len(),
                model.fields.len()
            ));
        }

        let id = self.get_id();
        let mut row = Row {
            id,
            entries: vec![],
        };

        for colum in &self.colums {
            let field = model.find_field(&colum.name);

            if field.is_none() {
                return Err(format!("[Error]: Missing field '{}'", &colum.name));
            }

            if field.clone().unwrap().type_ != colum.type_ {
            return Err(format!(
                "[Error]: Field '{}' must be of type '{}' found type '{}'",
                &colum.name,
                &colum.type_,
                field.unwrap().type_,
            ));
            }

            row.entries.push(field.unwrap().value.clone());
        }

        self.rows.push(row);
        Ok(id)
    }
}

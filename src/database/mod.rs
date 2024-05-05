use core::fmt;

use crate::ast::{DataType, Model, Object, Schema, SchemaField};

#[derive(Clone)]
pub enum Condition {
    IsEqual(Object),
    NotEqual(Object),
    LessThan(Object),
    GratherThan(Object),
    LessOrEqual(Object),
    GratherOrEqual(Object),
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IsEqual(_) => write!(f, "=="),
            Self::NotEqual(_) => write!(f, "!="),
            Self::LessThan(_) => write!(f, "<"),
            Self::LessOrEqual(_) => write!(f, "<="),
            Self::GratherThan(_) => write!(f, ">"),
            Self::GratherOrEqual(_) => write!(f, ">="),
        }
    }
}

pub struct WhereQuery {
    pub field: String,
    pub cond: Condition,
}

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

    pub fn query_by_id(&self, id: usize) -> Option<&Row> {
        if id <= 0 || id >= self.next_id {
            return None;
        }
        self.rows.iter().find(|r| r.id == id)
    }

    pub fn query_where(&self, q: WhereQuery) -> Result<Vec<&Row>, String> {
        let column_pos = self.colums.iter().position(|c| c.name == q.field);

        if column_pos.is_none() {
            return Err(format!(
                "[Error]: table '{}' has no field named '{}'",
                self.name, q.field
            ));
        }

        let column_pos = column_pos.unwrap();

        if !Self::datatype_support_condition(&self.colums[column_pos].type_, &q.cond.clone()) {
            return Err(format!(
                "[Error]: Condition '{}' is not impl for DataType '{}'",
                q.cond, self.colums[column_pos].type_
            ));
        }

        let rows = self
            .rows
            .iter()
            .filter(|x| Self::apply_condition(&x.entries[column_pos], &q.cond))
            .collect::<Vec<&Row>>();

        Ok(rows)
    }

    fn apply_condition(lhs: &Object, cond: &Condition) -> bool {
        match cond {
            Condition::IsEqual(rhs) => lhs == rhs,
            Condition::NotEqual(rhs) => lhs != rhs,
            Condition::LessThan(rhs) => lhs < rhs,
            Condition::GratherThan(rhs) => lhs > rhs,
            Condition::LessOrEqual(rhs) => lhs <= rhs,
            Condition::GratherOrEqual(rhs) => lhs >= rhs,
        }
    }

    fn datatype_support_condition(type_: &DataType, cond: &Condition) -> bool {
        match type_ {
            DataType::String => match cond {
                Condition::IsEqual(_) => true,
                Condition::NotEqual(_) => true,
                _ => false,
            },
            DataType::Number => match cond {
                Condition::IsEqual(_) => true,
                Condition::NotEqual(_) => true,
                Condition::LessThan(_) => true,
                Condition::GratherThan(_) => true,
                Condition::LessOrEqual(_) => true,
                Condition::GratherOrEqual(_) => true,
            },
        }
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

mod ast;
mod lexer;
mod parser;
mod token;
mod database;

use database::Table;
use parser::Parser;

fn main() {
    let schema = r#"
        "users": {
            "name": String,
            "age": Number,
            "phone": String,
        }
        "#;

    let input = schema.chars().collect::<Vec<char>>();
    let schema = Parser::parse_schema(&input);

    let model1 = r#"
        "users": {
            "name": "Edilson",
            "age": 22,
            "phone": "+55 123-4567",
        }
        "#;

    let model2 = r#"
        "users": {
            "name": "Mungoi",
            "age": 26,
            "phone": "+244 123-4567",
        }
        "#;

    let model3 = r#"
        "users": {
            "name": "Grahms",
            "age": 24,
            "phone": "+34 123-4567",
        }
        "#;

    let input = model1.chars().collect::<Vec<char>>();
    let model1 = Parser::parse_model(&input);

    let input = model2.chars().collect::<Vec<char>>();
    let model2 = Parser::parse_model(&input);

    let input = model3.chars().collect::<Vec<char>>();
    let model3 = Parser::parse_model(&input);

    let mut table = Table::new(schema.unwrap());

    let _ = table.add_row(model1.unwrap());
    let _ = table.add_row(model2.unwrap());
    let _ = table.add_row(model3.unwrap());

    println!("{:#?}", table.get_by_id(1));
}

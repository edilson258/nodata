mod ast;
mod lexer;
mod parser;
mod token;

use parser::Parser;

fn main() {
    let schema = r#"
        "users": {
            "name": String,
            "age": Number,
            "phone": String,
        }
        "#;

    let data = r#"
        "users": {
            "name": "Edilson",
            "age": 22,
            "phone": "Hello",
        }
        "#;

    let input = data.chars().collect::<Vec<char>>();
    let model_data = Parser::parse_data(&input);
    if model_data.is_err() {
        println!("{}", model_data.err().unwrap());
    } else {
        println!("{:#?}", model_data.ok().unwrap());
    }

    let input = schema.chars().collect::<Vec<char>>();
    let model_schema = Parser::parse_schema(&input);
    if model_schema.is_err() {
        println!("{}", model_schema.err().unwrap());
    } else {
        println!("{:#?}", model_schema.ok().unwrap());
    }

}

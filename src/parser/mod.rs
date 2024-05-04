use crate::ast::{DataType, Field, FieldValue, Model};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    l: Lexer<'a>,
    curr_token: Token,
    next_token: Token,
    is_schema: bool,
}

impl<'a> Parser<'a> {
    pub fn parse_schema(input: &'a [char]) -> Result<Model, String> {
        let mut p = Self::new(input, true);
        p.parse()
    }

    pub fn parse_data(input: &'a [char]) -> Result<Model, String> {
        let mut p = Self::new(input, false);
        p.parse()
    }

    fn new(input: &'a [char], is_schema: bool) -> Self {
        let mut p = Parser {
            l: Lexer::new(&input),
            curr_token: Token::Eof,
            next_token: Token::Eof,
            is_schema,
        };

        p.read_token();
        p.read_token();

        p
    }

    fn read_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.l.next_token();
    }

    fn parse(&mut self) -> Result<Model, String> {
        let name = match self.curr_token.clone() {
            Token::String(val) => val,
            _ => return Err("[Error]: Invalid model name".to_string()),
        };
        self.read_token();

        if !self.skip(Token::Colon) {
            return Err(format!(
                "[Error]: Expected ':' but found {}",
                self.curr_token
            ));
        }

        let fields = self.parse_fields();

        if fields.is_err() {
            return Err(fields.err().unwrap());
        }

        Ok(Model {
            name,
            fields: fields.ok().unwrap(),
        })
    }

    fn parse_fields(&mut self) -> Result<Vec<Field>, String> {
        if !self.skip(Token::Lbrace) {
            return Err(format!(
                "[Error]: Expected '{{' but found {}",
                self.curr_token
            ));
        }

        let mut fields: Vec<Field> = vec![];

        loop {
            if self.curr_token_is(Token::Eof) {
                return Err("[Error]: Unexpected EOF".to_string());
            }

            if self.curr_token_is(Token::Rbrace) {
                break;
            }

            let field = self.parse_field();
            if field.is_err() {
                return Err(field.err().unwrap());
            }
            fields.push(field.ok().unwrap());

            if !self.skip(Token::Comma) {
                if !self.curr_token_is(Token::Rbrace) {
                    return Err("[Error]: Expected ',' or '{{'".to_string());
                }
            }
        }

        Ok(fields)
    }

    fn parse_field(&mut self) -> Result<Field, String> {
        let name = match self.curr_token.clone() {
            Token::String(val) => val,
            _ => return Err(format!("[Error]: Invalid field name")),
        };
        self.read_token();

        if !self.skip(Token::Colon) {
            return Err(format!(
                "[Error]: Expected ':' but found {}",
                self.curr_token
            ));
        }

        let value = match self.is_schema {
            true => match self.curr_token.clone() {
                Token::TypeNumber => FieldValue::DataType(DataType::Number),
                Token::TypeString => FieldValue::DataType(DataType::String),
                _ => return Err(format!("[Error]: Invalid field value {}", self.curr_token)),
            },
            false => match self.curr_token.clone() {
                Token::String(val) => FieldValue::String(val),
                Token::Number(val) => FieldValue::Number(val),
                _ => return Err(format!("[Error]: Invalid field value {}", self.curr_token)),
            },
        };

        self.read_token();
        Ok(Field { name, value })
    }

    fn curr_token_is(&mut self, tk: Token) -> bool {
        self.curr_token == tk
    }

    fn skip(&mut self, tk: Token) -> bool {
        if self.curr_token != tk {
            return false;
        }
        self.read_token();
        return true;
    }
}

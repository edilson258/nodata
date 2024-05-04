use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    l: Lexer<'a>,
    curr_token: Token,
    next_token: Token,
}

impl<'a> Parser<'a> {
    /* Model  */

    pub fn parse_model(input: &'a [char]) -> Result<Model, String> {
        let mut p = Self::new(input);

        let name = match p.curr_token.clone() {
            Token::String(val) => val,
            _ => return Err("[Error]: Invalid model name".to_string()),
        };
        p.read_token();

        if !p.skip(Token::Colon) {
            return Err(format!(
                "[Error]: Expected ':' but found {}",
                p.curr_token
            ));
        }

        let fields = p.parse_model_fields();

        if fields.is_err() {
            return Err(fields.err().unwrap());
        }

        Ok(Model {
            name,
            fields: fields.ok().unwrap(),
        })
    }

    fn parse_model_fields(&mut self) -> Result<Vec<ModelField>, String> {
        if !self.skip(Token::Lbrace) {
            return Err(format!(
                "[Error]: Expected '{{' but found {}",
                self.curr_token
            ));
        }

        let mut fields: Vec<ModelField> = vec![];

        loop {
            if self.curr_token_is(Token::Eof) {
                return Err("[Error]: Unexpected EOF".to_string());
            }

            if self.curr_token_is(Token::Rbrace) {
                break;
            }

            let field = self.parse_model_field();
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

    fn parse_model_field(&mut self) -> Result<ModelField, String> {
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

        let (value, type_) = match self.curr_token.clone() {
            Token::String(val) => (Object::String(val), DataType::String),
            Token::Number(val) => (Object::Number(val), DataType::Number),
            _ => return Err(format!("[Error]: Invalid field value {}", self.curr_token)),
        };

        self.read_token();
        Ok(ModelField { name, value, type_ })
    }


    /* Schema  */

    pub fn parse_schema(input: &'a [char]) -> Result<Schema, String> {
        let mut p = Self::new(input);

        let name = match p.curr_token.clone() {
            Token::String(val) => val,
            _ => return Err("[Error]: Invalid schema name".to_string()),
        };
        p.read_token();

        if !p.skip(Token::Colon) {
            return Err(format!("[Error]: Expected ':' but found {}", p.curr_token));
        }

        let fields = p.parse_schema_fields();

        if fields.is_err() {
            return Err(fields.err().unwrap());
        }

        Ok(Schema {
            name,
            fields: fields.ok().unwrap(),
        })
    }

    fn parse_schema_fields(&mut self) -> Result<Vec<SchemaField>, String> {
        if !self.skip(Token::Lbrace) {
            return Err(format!(
                "[Error]: Expected '{{' but found {}",
                self.curr_token
            ));
        }

        let mut fields: Vec<SchemaField> = vec![];

        loop {
            if self.curr_token_is(Token::Eof) {
                return Err("[Error]: Unexpected EOF".to_string());
            }

            if self.curr_token_is(Token::Rbrace) {
                break;
            }

            let field = self.parse_schema_field();
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

    fn parse_schema_field(&mut self) -> Result<SchemaField, String> {
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

        let type_ = match self.curr_token.clone() {
            Token::TypeNumber => DataType::Number,
            Token::TypeString => DataType::String,
            _ => return Err(format!("[Error]: Invalid field value {}", self.curr_token)),
        };

        self.read_token();
        Ok(SchemaField { name, type_ })
    }

    /* Helpers */

    fn new(input: &'a [char]) -> Self {
        let mut p = Parser {
            l: Lexer::new(&input),
            curr_token: Token::Eof,
            next_token: Token::Eof,
        };

        p.read_token();
        p.read_token();

        p
    }

    fn read_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.l.next_token();
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

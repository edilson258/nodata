use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Eof,
    Illegal(String),

    Lbrace,
    Rbrace,
    Comma,
    Colon,

    TypeNumber,
    TypeString,

    Number(f64),
    String(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Eof => write!(f, "EOF"),
            Self::Comma => write!(f, ","),
            Self::Number(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
            Self::Lbrace => write!(f, "{{"),
            Self::Rbrace => write!(f, "}}"),
            Self::Colon => write!(f, ":"),
            Self::TypeNumber => write!(f, "[Type Annotation] Number"),
            Self::TypeString => write!(f, "[Type Annotation] String"),
            Self::Illegal(val) => write!(f, "[Illegal Token] {}", val),
        }
    }
}

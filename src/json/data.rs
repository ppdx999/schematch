#[derive(Debug, PartialEq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "start: {}, end: {}", self.start, self.end)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Colon,
    Comma,
    LeftBrace,
    RightBrace,
    LessThan,
    GreaterThan,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
    pub next: Option<Box<Token>>,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Null,
    String,
    Number,
    Boolean,
    Object(Box<Object>),
    Array(Box<Array>),
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub type_: Type,
}

impl Property {
    pub fn new(name: String, type_: Type) -> Property {
        Property {
            name,
            type_,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub properties: Vec<Property>,
}

impl Object {
    pub fn new() -> Object {
        Object { properties: vec![] }
    }
}

#[derive(Debug, PartialEq)]
pub struct Array {
    pub type_: Type,
}

impl Array {
    pub fn new(type_: Type) -> Array {
        Array { type_ }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Object(Object),
    Array(Array),
    Type(Type),
}

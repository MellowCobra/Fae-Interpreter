use interpreter::data_type::Type;

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: Type,
    pub value: i32
}

impl Token {
    pub fn new(_type: Type, value: i32) -> Token {
        Token {
            _type,
            value
        }
    }
}
pub mod parsing_error {
    pub fn throw(line: i32, location: usize) {
        panic!("Parsing exception occured on line {} at location {}", line, location);
    }
}

pub mod unknown_token_error {
    pub fn throw(line: i32, location: usize, token: char) {
        panic!("Parsing exception occured on line {} at location {}: Unknown token {}", line, location, token);
    }
}

pub mod unmatched_token_error {
    use interpreter::data_type::Type;
    pub fn throw(line: i32, location: usize, expected: &Type, actual: &Type) {
        panic!("Parsing exception occured on line {} at location {}: Unmatched token - expected {:?} but got {:?} instead", line, location, expected, actual);
    }
}
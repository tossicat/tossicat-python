use std::fmt;


#[derive(Debug, PartialEq)]
pub enum InvalidValue {
    InvalidTossi,
    LimitLength
}

impl fmt::Display for InvalidValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidValue::InvalidTossi => write!(f, "This value is not correct tossi"),
            InvalidValue::LimitLength => write!(f, "The length has been exceeded. Set the word length to less than 50.")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ValueError {
    error: InvalidValue,
    description: String, 
}


impl ValueError {
    pub fn new(error: InvalidValue) -> Self {
        ValueError {
            description: error.to_string(),
            error,
        }
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

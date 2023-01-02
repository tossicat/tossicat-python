use std::fmt;


#[derive(Debug, PartialEq)]
pub enum ValueError {
    InvalidTossi,
    LimitLength
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueError::InvalidTossi => write!(f, "This value is not correct tossi"),
            ValueError::LimitLength => write!(f, "The length has been exceeded. Set the word length to less than 50.")
        }
    }
}


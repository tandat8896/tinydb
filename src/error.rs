#[derive(Debug)] // in log  error

pub enum DbError {
    Io(std::io::Error), // error io
    Parse(String),      // error parse so , string
    NotFound(String),   //error khong tim thay row, id
    InvalidQuery(String),
}

impl From<std::io::Error> for DbError {
    fn from(e: std::io::Error) -> Self {
        DbError::Io(e)
    }
    // add code here
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::Io(e) => write!(f, "IO: {}", e),
            DbError::Parse(s) => write!(f, "Parse: {}", s),
            DbError::NotFound(s) => write!(f, "NotFound: {}", s),
            DbError::InvalidQuery(s) => write!(f, "InvalidInvalidQuery: {}", s),
        }
    }
}

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ParsingError {
    MissingFields,
    InvalidFormat,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Parsing error: {}", 
            match self {
                ParsingError::MissingFields => "MissingFields",
                ParsingError::InvalidFormat => "InvalidFormat",
        })
    }
}

impl std::error::Error for ParsingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}
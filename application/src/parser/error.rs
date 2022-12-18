use std::fmt::Display;

// TODO: Сделать более подробные ошибки
#[derive(Debug, Clone)]
pub enum ParsingError {
    MissingFields,
    InvalidFormat, 
    TypeMismatch, 
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Parsing error: {}", 
            match self {
                ParsingError::MissingFields => "MissingFields",
                ParsingError::InvalidFormat => "InvalidFormat",
                ParsingError::TypeMismatch => "TypeMismatch"
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
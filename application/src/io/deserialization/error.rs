/// Модуль, хранящий тип ошибки сериализации.
use std::fmt::Display;

// TODO: Сделать более подробные ошибки
#[derive(Debug, Clone)]
pub enum ParsingError {
    MissingField(String),
    InvalidFormat,
    TypeMismatch,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Parsing error: {}",
            match self {
                ParsingError::MissingField(s) => format!("MissingField: {s}"),
                ParsingError::InvalidFormat => "InvalidFormat".to_string(),
                ParsingError::TypeMismatch => "TypeMismatch".to_string(),
            }
        )
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

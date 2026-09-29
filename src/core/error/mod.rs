use std::fmt;
use std::error::Error;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum ZigurateError {
    IOError(String),
    ByteConversionFailed(Utf8Error),
    SerdeError(serde_yaml::Error),
    UnexpectedSymbol,
    EnkiFunctionNotFound(String),
    UdaNotFound(String),
    LunigNotFound(String),
    LunigPropertieLimitExceeded(String),
    SyntaxError(String),
    LunigAlreadyExist(String),
    ProjectNotFoundOrNotExist,
    UnknownCallFunctionName(String),
    PropertiesExpected(String)
}


impl fmt::Display for ZigurateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            ZigurateError::IOError(msg) => write!(f, "{}", msg),
            ZigurateError::ByteConversionFailed(msg) => write!(f, "{}", msg),
            ZigurateError::UnexpectedSymbol => write!(f, "Unexpected symbol found."),
            ZigurateError::EnkiFunctionNotFound(msg) => write!(f, "{} not exist or not found", msg),
            ZigurateError::UdaNotFound(msg) => write!(f, "{} not exist or not found", msg),
            ZigurateError::LunigNotFound(msg) => write!(f, "{} not exist or not found", msg),
            ZigurateError::SyntaxError(msg) => write!(f, "{}", msg),
            ZigurateError::LunigAlreadyExist(msg) => write!(f, "{}", msg),
            ZigurateError::ProjectNotFoundOrNotExist => write!(f, "Project not found or not exists"),
            ZigurateError::SerdeError(e) => write!(f, "{}", e),
            ZigurateError::UnknownCallFunctionName(e) => write!(f, "{}", e),
            ZigurateError::PropertiesExpected(e) => write!(f, "{}", e),
            ZigurateError::LunigPropertieLimitExceeded(e) => write!(f, "{}", e),
            
        }
    }
}



impl Error for ZigurateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ZigurateError::ByteConversionFailed(err) => Some(err),

            ZigurateError::IOError(_) => None,
            ZigurateError::UnexpectedSymbol => None,
            ZigurateError::EnkiFunctionNotFound(_) => None,
            ZigurateError::UdaNotFound(_) => None,
            ZigurateError::LunigNotFound(_) => None,
            ZigurateError::SyntaxError(_) => None,
            ZigurateError::LunigAlreadyExist(_) => None,
            ZigurateError::ProjectNotFoundOrNotExist => None,
            ZigurateError::SerdeError(_) => None,
            ZigurateError::UnknownCallFunctionName(_) => None,
            ZigurateError::PropertiesExpected(_) => None,
            ZigurateError::LunigPropertieLimitExceeded(_) => None,
            
        }
    }
}



impl From<Utf8Error> for ZigurateError {
    fn from(err: Utf8Error) -> Self {
        ZigurateError::ByteConversionFailed(err)
    }
}

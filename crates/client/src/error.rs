use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    Connection(String),
    Timeout,
    Protocol(String),
    NotFound,
    Io(std::io::Error),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::Connection(msg) => write!(f, "connection error: {}", msg),
            ClientError::Timeout => write!(f, "request timeout"),
            ClientError::Protocol(msg) => write!(f, "protocol error: {}", msg),
            ClientError::NotFound => write!(f, "key not found"),
            ClientError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClientError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ClientError {
    fn from(err: std::io::Error) -> Self {
        ClientError::Io(err)
    }
}

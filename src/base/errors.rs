/// A typedef of the result returned by many methods.
pub type Result<T> = core::result::Result<T, RabpError>;

/// Error with multiple strings.
#[derive(Debug)]
pub struct MessagesError {
    errors: Vec<String>,
}

impl std::error::Error for MessagesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for MessagesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut not1 = false;
        for x in self.errors.iter() {
            if not1 {
                write!(f, "\n")?;
            } else {
                not1 = true;
            }
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RabpError {
    /// Error from diesel.
    DieselError {
        source: diesel::result::Error,
    },
    NotFound,
    /// Error from services.
    ServiceError {
        source: MessagesError,
    },
}

impl std::error::Error for RabpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RabpError::DieselError { ref source } => Some(source),
            RabpError::NotFound => None,
            RabpError::ServiceError { ref source } => Some(source),
        }
    }
}

impl std::fmt::Display for RabpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RabpError::DieselError { ref source } => write!(f, "Diesel error ({})", source),
            RabpError::NotFound => write!(f, "Not found"),
            RabpError::ServiceError { ref source } => write!(f, "{}", source),
        }
    }
}

impl From<diesel::result::Error> for RabpError {
    fn from(source: diesel::result::Error) -> RabpError {
        RabpError::DieselError { source }
    }
}

impl RabpError {
    /// Returns error from string list.
    pub fn error(r: &Vec<String>) -> RabpError {
        RabpError::ServiceError {
            source: MessagesError { errors: r.clone() },
        }
    }

    /// Returns error from string.
    pub fn error_string(r: &str) -> RabpError {
        let v = vec![r.to_string()];
        RabpError::ServiceError {
            source: MessagesError { errors: v },
        }
    }

    // /// Returns error from message.
    // pub fn error_msg(key: M, is_de: bool) -> RabpError {
    //     let r = M::mec(key, is_de).to_owned().to_string();
    //     let v = vec![r];
    //     RsbpError::ServiceError {
    //         source: MessagesError { errors: v },
    //     }
    // }
}

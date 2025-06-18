use thiserror::Error;

/// Validation errors for data model validation
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Field '{0}' cannot be empty")]
    EmptyField(String),

    #[error("Field '{field}' value {value} is out of range [{min}, {max}]")]
    OutOfRange {
        field: String,
        value: f64,
        min: f64,
        max: f64,
    },

    #[error("Invalid format for field '{field}': {message}")]
    InvalidFormat { field: String, message: String },

    #[error("Required field '{0}' is missing")]
    MissingField(String),
}

/// Trait for validating data models
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

/// Helper function to validate that a string field is not empty
pub fn validate_non_empty_string(field_name: &str, value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        Err(ValidationError::EmptyField(field_name.to_string()))
    } else {
        Ok(())
    }
}

/// Helper function to validate that a string contains only alphanumeric characters and underscores
pub fn validate_id_format(field_name: &str, value: &str) -> Result<(), ValidationError> {
    validate_non_empty_string(field_name, value)?;

    if !value.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::InvalidFormat {
            field: field_name.to_string(),
            message: "must contain only alphanumeric characters and underscores".to_string(),
        });
    }

    Ok(())
}

/// Helper function to validate numeric ranges
pub fn validate_range(
    field_name: &str,
    value: f64,
    min: f64,
    max: f64,
) -> Result<(), ValidationError> {
    if !(min..=max).contains(&value) {
        Err(ValidationError::OutOfRange {
            field: field_name.to_string(),
            value,
            min,
            max,
        })
    } else {
        Ok(())
    }
}

/// Helper function to validate that a timestamp is not in the future
pub fn validate_timestamp_not_future(
    field_name: &str,
    timestamp: &chrono::DateTime<chrono::Utc>,
) -> Result<(), ValidationError> {
    let now = chrono::Utc::now();
    if timestamp > &now {
        Err(ValidationError::InvalidFormat {
            field: field_name.to_string(),
            message: "timestamp cannot be in the future".to_string(),
        })
    } else {
        Ok(())
    }
}

/// Helper function to validate string length
pub fn validate_string_length(
    field_name: &str,
    value: &str,
    max_length: usize,
) -> Result<(), ValidationError> {
    if value.len() > max_length {
        Err(ValidationError::InvalidFormat {
            field: field_name.to_string(),
            message: format!("must be no more than {max_length} characters"),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_validate_non_empty_string() {
        assert!(validate_non_empty_string("test", "valid").is_ok());
        assert!(validate_non_empty_string("test", "").is_err());
    }

    #[test]
    fn test_validate_id_format() {
        assert!(validate_id_format("id", "valid_id_123").is_ok());
        assert!(validate_id_format("id", "invalid-id").is_err());
        assert!(validate_id_format("id", "invalid id").is_err());
        assert!(validate_id_format("id", "").is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range("temp", 25.0, 0.0, 100.0).is_ok());
        assert!(validate_range("temp", -1.0, 0.0, 100.0).is_err());
        assert!(validate_range("temp", 101.0, 0.0, 100.0).is_err());
    }

    #[test]
    fn test_validate_timestamp_not_future() {
        let now = Utc::now();
        let past = now - Duration::hours(1);
        let future = now + Duration::hours(1);

        assert!(validate_timestamp_not_future("timestamp", &past).is_ok());
        assert!(validate_timestamp_not_future("timestamp", &now).is_ok());
        assert!(validate_timestamp_not_future("timestamp", &future).is_err());
    }

    #[test]
    fn test_validate_string_length() {
        assert!(validate_string_length("message", "short", 10).is_ok());
        assert!(validate_string_length("message", "this is too long", 10).is_err());
    }
}

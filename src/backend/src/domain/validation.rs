use lazy_static::lazy_static;
use regex::Regex;
use validator::ValidationError;

lazy_static! {
    static ref ALPHANUM: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
}

pub fn validate_alphanum(input: &str) -> Result<(), ValidationError> {
    if ALPHANUM.is_match(input) {
        Ok(())
    } else {
        Err(ValidationError::new("alphanum"))
    }
}

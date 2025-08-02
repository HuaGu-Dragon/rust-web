use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;
use validator::ValidationError;

static MOBILE_PHONE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^1[3-9]\d{9}$").expect("Failed to compile mobile phone regex"));

pub fn validate_mobile_phone(number: &str) -> Result<(), ValidationError> {
    if MOBILE_PHONE.is_match(number) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_mobile_phone")
            .with_message(Cow::Borrowed("Invalid mobile phone number format")))
    }
}

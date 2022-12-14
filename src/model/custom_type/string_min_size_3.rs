use derive_more::Display;

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::exception::{ApiFieldError, ERR_MIN_SIZE};

#[derive(Display, Debug, Clone, Serialize, Deserialize, sqlx::Decode, sqlx::Encode)]
#[serde(try_from = "String", into = "String")]
pub struct StringMinSize3(String);

impl From<StringMinSize3> for String {
    fn from(s: StringMinSize3) -> String {
        s.0
    }
}

impl FromStr for StringMinSize3 {
    type Err = ApiFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StringMinSize3(s.to_string()))
    }
}

impl TryFrom<String> for StringMinSize3 {
    type Error = ApiFieldError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl StringMinSize3 {
    pub fn value(&self) -> String {
        self.0.to_owned()
    }

    pub fn validate(&self, field: String) -> Result<(), ApiFieldError> {
        if self.0.len() < 3 {
            return Err(ApiFieldError::new_with_min_size(
                ERR_MIN_SIZE,
                field,
                3,
            ));
        }

        Ok(())
    }
}

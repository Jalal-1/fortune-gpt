use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

use crate::domain::manifest::ManifestError;

/// The description field for a [`Manifest`](crate::domain::manifest::Manifest).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Description(String);

impl Description {
    /// Create a new `Description` field.
    ///
    /// If the description provided is empty, then a [`ManifestError`] will be returned.
    pub fn new(description: &str) -> Result<Self, ManifestError> {
        if !description.trim().is_empty() {
            Ok(Self(description.to_owned()))
        } else {
            Err(ManifestError::EmptyDescription)
        }
    }
    /// Return the underlying [`String`].
    pub fn into_inner(self) -> String {
        self.0
    }
    /// Return a reference to the underlying [`&str`].
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Description {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value).map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}

#[cfg(test)]
mod test {
    use super::Description;

    #[test]
    fn disallow_empty_description() {
        assert!(Description::new("").is_err());
    }
}

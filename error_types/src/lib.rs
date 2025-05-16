use chrono::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    form_values: (String, String),
    date: String,
    err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new(
                self.name.clone(), 
                self.password.clone(),
                "Username is empty",
            ));
        } else if self.password.chars().count() < 8 {
            return Err(FormError::new(
                self.name.clone(), 
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        let mut count_ascii = 0;
        let mut count_number = 0;
        let mut count_symbol = 0;

        for ch in self.password.chars() {
            if ch.is_alphabetic() {
                count_ascii += 1;
            } else if ch.is_numeric() {
                count_number += 1;
            } else {
                count_symbol += 1;
            }
        }

        if count_ascii == 0 || count_number == 0 || count_symbol == 0 {
            return Err(FormError::new(
                self.name.clone(), 
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }
        Ok(())
    }
}

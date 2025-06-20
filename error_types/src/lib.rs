pub type Utc = chrono::Utc;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, password: String) -> Self {
        Self {
            name: name,
            password: password,
        }
    }

    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ))
        } else if self.password.chars().count() < 8 {
            Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ))
        } else if self.password.chars().count() < 8 {
            Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ))
        } else {
            let (mut count_ascii, mut count_number, mut count_symbol) = (0, 0, 0);

            for ch in self.password.chars() {
                match ch {
                    ch if ch.is_alphabetic() => count_ascii += 1,
                    ch if ch.is_numeric() => count_number += 1,
                    _ => count_symbol += 1,
                }

                // Early exit optimization if all three types are present
                if count_ascii > 0 && count_number > 0 && count_symbol > 0 {
                    break;
                }
            }

            if count_ascii == 0 || count_number == 0 || count_symbol == 0 {
                Err(FormError::new(
                    "password",
                    self.password.clone(),
                    "Password must include letters, numbers, and symbols.",
                ))
            } else {
                Ok(())
            }
        }
    }
}

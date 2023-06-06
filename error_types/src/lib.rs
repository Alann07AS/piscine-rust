pub use chrono::{Utc, NaiveDate};

pub fn create_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}


const ERR_DESCRIPTION_USER_NAME: &str = "No user name";
const ERR_DESCRIPTION_PWD_LEN: &str = "At least 8 characters";
const ERR_DESCRIPTION_PWD_FORMAT: &str = "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)";


// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),  //this will be a tuple of strings representing the invalid input. For example: ("password", "asdaSD\_") or ("first_name", "someone")
    pub date: String,   //that will have the date that the error occurred in the format "2020-12-14 09:33:41"
    pub err: String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date:  Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err
        }
    }
    fn is_password_format_correct(password: &str) -> bool {
        password.chars().any(|ch| ch.is_ascii_alphabetic())     &&      //ascii alphabet
        password.chars().any(|ch| ch.is_ascii_digit())         &&      //ascii_digit
        password.chars().any(|ch| !ch.is_ascii_alphanumeric())         //ascii special
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate, //that will convert a string "2015-09-05" to a date of that format.
    pub birth_location: String,
    pub password: String
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth, //that will convert a string "2015-09-05" to a date of that format.
            birth_location,
            password
        }
    }
    
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        if self.first_name.is_empty() {
            Err(FormError::new("first_name".to_string(), self.first_name.clone(), ERR_DESCRIPTION_USER_NAME.to_string()))
        } else if self.password.chars().count() < 8 {
            Err(FormError::new("password".to_string(), self.password.clone(), ERR_DESCRIPTION_PWD_LEN.to_string()))
        } else if !FormError::is_password_format_correct(&self.password) {
            Err(FormError::new("password".to_string(), self.password.clone(), ERR_DESCRIPTION_PWD_FORMAT.to_string()))
        } else {
            Ok(vec!["Valid first name", "Valid password"])
        }
        //Err(FormError::new("".to_string(), "".to_string(), "".to_string()))
    }
}

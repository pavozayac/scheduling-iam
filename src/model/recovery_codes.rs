use chrono::NaiveDateTime;
use derive_builder::Builder;
use uuid::Uuid;
use validator::{self, ValidateLength};

#[derive(Debug, Builder, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct RecoveryCode {
    user_id: Uuid,
    #[builder(default = "Self::generate_recovery_code()")]
    code: String,
    #[builder(default = "false")]
    active: bool,
    #[builder(default = "chrono::Utc::now().naive_utc()")]
    date_created: NaiveDateTime,
}

impl RecoveryCode {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn active(&self) -> bool {
        self.active
    }
}

impl RecoveryCodeBuilder {
    fn generate_code_character() -> char {
        let capitalized = rand::random_bool(0.5);

        if capitalized {
            (rand::random_range(0..25) + 65) as u8 as char
        } else {
            (rand::random_range(0..25) + 97) as u8 as char
        }
    }

    pub fn generate_recovery_code() -> String {
        let arr = (0..16).map(|_| Self::generate_code_character());

        String::from_iter(arr)
    }

    fn validate(&self) -> Result<(), String> {
        match self.code {
            Some(ref code) if code.validate_length(None, None, 16.into()) => Ok(()),
            None => Ok(()),
            _ => Err("Invalid recovery code length".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_with_correct_length_code() {
        let user_id = Uuid::new_v4();
        let code = "testcode12345678";

        let recovery_code = RecoveryCodeBuilder::default()
            .user_id(user_id)
            .code(code.into())
            .build();

        assert!(recovery_code.is_ok());

        let recovery_code = recovery_code.unwrap();

        assert_eq!(code, recovery_code.code());
        assert_eq!(user_id, recovery_code.user_id());
    }

    #[test]
    fn should_not_build_with_short_code() {
        let user_id = Uuid::new_v4();
        let code = "testcode";

        let recovery_code = RecoveryCodeBuilder::default()
            .user_id(user_id)
            .code(code.into())
            .build();

        assert!(recovery_code.is_err());
    }

    #[test]
    fn should_build_with_default_16_length_code() {
        let user_id = Uuid::new_v4();

        let recovery_code = RecoveryCodeBuilder::default().user_id(user_id).build();

        assert!(recovery_code.is_ok());

        let recovery_code = recovery_code.unwrap();

        assert_eq!(16, recovery_code.code().len());
        assert_eq!(user_id, recovery_code.user_id());
    }
}

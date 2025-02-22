use anyhow::{anyhow, Context};
use derive_builder::Builder;
use thiserror::Error;
use uuid::{self, Uuid};
use validator::{Validate, ValidateEmail};

use super::{
    otp::OtpCode,
    recovery_codes::{RecoveryCode, RecoveryCodeBuilder, RecoveryCodeBuilderError},
};

const RECOVERY_CODE_COUNT: usize = 4;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Invalid recovery code")]
    InvalidRecoveryCode,
}

// The `User` will serve as the root aggregate for the model
#[derive(Debug, Default, Builder, Validate, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct User {
    user_id: Uuid,
    email: String,
    #[builder(default = "Vec::new()")]
    recovery_codes: Vec<RecoveryCode>,
    #[builder(default = "None")]
    otp: Option<OtpCode>,
}

impl User {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn recovery_codes(&self) -> &[RecoveryCode] {
        &self.recovery_codes
    }

    pub fn otp(&self) -> Option<&OtpCode> {
        self.otp.as_ref()
    }

    pub fn use_recovery_code(&self, code: &str) -> anyhow::Result<Self> {
        if !self.recovery_codes.iter().any(|rc| rc.code() == code) {
            return Err(anyhow!(UserError::InvalidRecoveryCode)).context("Recovery code not found");
        }

        let mut new_recovery_codes: Vec<RecoveryCode> = self
            .recovery_codes
            .iter()
            .filter(|rc| rc.code() != code)
            .cloned()
            .collect();

        new_recovery_codes.push(
            RecoveryCodeBuilder::default()
                .user_id(self.user_id)
                .build()?,
        );

        let new = User {
            recovery_codes: new_recovery_codes,
            ..self.clone()
        };

        Ok(new)
    }

    pub fn generate_new_recovery_codes(&self) -> anyhow::Result<Self> {
        let recovery_codes_results: Vec<Result<RecoveryCode, RecoveryCodeBuilderError>> = (0
            ..RECOVERY_CODE_COUNT)
            .map(|_| RecoveryCodeBuilder::default().user_id(self.user_id).build())
            .collect();

        if recovery_codes_results.iter().any(|res| res.is_err()) {
            return Err(anyhow!(UserError::InvalidRecoveryCode))
                .context("Failed to generate recovery codes");
        }

        let new = User {
            recovery_codes: recovery_codes_results
                .iter()
                .map(|res| res.as_ref().unwrap())
                .cloned()
                .collect(),
            ..self.clone()
        };

        Ok(new)
    }
}

impl UserBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.email.validate_email() {
            Ok(())
        } else {
            Err("Invalid email".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_with_correct_data() {
        let user_id = Uuid::new_v4();
        let email = "testuser@example.com";

        let user = UserBuilder::default()
            .user_id(user_id)
            .email(email.into())
            .build();

        assert!(user.is_ok());

        let user = user.unwrap();

        assert_eq!(user_id, user.user_id());
        assert_eq!(email, user.email());
    }

    #[test]
    fn should_not_build_with_wrong_email() {
        let user_id = Uuid::new_v4();
        let email = "testuser";

        let user = UserBuilder::default()
            .user_id(user_id)
            .email(email.into())
            .build();

        assert!(user.is_err());
    }

    #[test]
    fn should_generate_new_recovery_codes() {
        let user_id = Uuid::new_v4();
        let email = "testuser@example.com";

        let user = UserBuilder::default()
            .user_id(user_id)
            .email(email.into())
            .build()
            .unwrap();

        let user = user.generate_new_recovery_codes().unwrap();

        assert_eq!(user.recovery_codes().len(), RECOVERY_CODE_COUNT);
    }

    #[test]
    fn should_replenish_used_recovery_code() {
        let user_id = Uuid::new_v4();
        let email = "testuser@example.com";

        let user = UserBuilder::default()
            .user_id(user_id)
            .email(email.into())
            .build()
            .unwrap();

        let user = user.generate_new_recovery_codes().unwrap();
        let recovery_code = user.recovery_codes()[0].code().to_string();

        let user = user.use_recovery_code(&recovery_code).unwrap();

        assert_eq!(user.recovery_codes().len(), RECOVERY_CODE_COUNT);
        assert!(user
            .recovery_codes()
            .iter()
            .all(|rc| rc.code() != recovery_code));
    }

    #[test]
    fn should_fail_to_use_invalid_recovery_code() {
        let user_id = Uuid::new_v4();
        let email = "testuser@example.com";

        let user = UserBuilder::default()
            .user_id(user_id)
            .email(email.into())
            .build()
            .unwrap();

        let result = user.generate_new_recovery_codes();

        assert!(result.is_ok());

        let invalid_code = "invalid_code";

        let use_recovery_code_result = user.use_recovery_code(invalid_code);

        assert!(use_recovery_code_result.is_err());
        assert!(use_recovery_code_result
            .unwrap_err()
            .downcast_ref::<UserError>()
            .is_some());
    }
}

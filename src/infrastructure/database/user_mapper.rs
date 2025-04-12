use crate::domain::{
    otp::{OtpCode, OtpCodeBuilder},
    recovery_codes::{RecoveryCode, RecoveryCodeBuilder},
    user::{User, UserBuilder},
};

pub struct PsqlUserMapper;

impl PsqlUserMapper {
    pub fn map_user_query_result(
        user_query: UserQueryResult,
        otp_code_query: Option<OtpCodeQueryResult>,
        recovery_codes_query: &[RecoveryCodeQueryResult],
    ) -> anyhow::Result<User> {
        let created_datetime = user_query.date_created.and_utc();
        let updated_datetime = user_query.date_updated.and_utc();

        let recovery_codes = recovery_codes_query
            .iter()
            .map(|rc| rc.try_into())
            .collect::<anyhow::Result<Vec<RecoveryCode>>>()?;

        let otp = match otp_code_query {
            Some(otp) => Some(otp.try_into()?),
            None => None,
        };

        let user = UserBuilder::default()
            .user_id(user_query.id)
            .email(user_query.email)
            .registered(user_query.registered)
            .active_otp(otp)
            .active_recovery_codes(recovery_codes)
            .date_created(created_datetime)
            .date_updated(updated_datetime)
            .build()?;

        Ok(user)
    }
}

#[derive(Debug)]
pub struct UserQueryResult {
    pub id: uuid::Uuid,
    pub email: String,
    pub registered: bool,
    pub date_created: chrono::NaiveDateTime,
    pub date_updated: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct RecoveryCodeQueryResult {
    pub user_id: uuid::Uuid,
    pub code: String,
    pub date_created: chrono::NaiveDateTime,
    pub date_updated: chrono::NaiveDateTime,
}

impl TryFrom<RecoveryCodeQueryResult> for RecoveryCode {
    type Error = anyhow::Error;

    fn try_from(value: RecoveryCodeQueryResult) -> anyhow::Result<RecoveryCode> {
        let recovery_code = RecoveryCodeBuilder::default()
            .user_id(value.user_id)
            .code(value.code)
            .date_created(value.date_created.and_utc())
            .date_updated(value.date_updated.and_utc())
            .build()?;

        Ok(recovery_code)
    }
}

impl TryFrom<&RecoveryCodeQueryResult> for RecoveryCode {
    type Error = anyhow::Error;

    fn try_from(value: &RecoveryCodeQueryResult) -> anyhow::Result<RecoveryCode> {
        let recovery_code = RecoveryCodeBuilder::default()
            .user_id(value.user_id)
            .code(value.code.clone())
            .date_created(value.date_created.and_utc())
            .date_updated(value.date_updated.and_utc())
            .build()?;

        Ok(recovery_code)
    }
}

#[derive(Debug)]
pub struct OtpCodeQueryResult {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub code: String,
    pub expires_at: chrono::NaiveDateTime,
    pub date_created: chrono::NaiveDateTime,
    pub date_updated: chrono::NaiveDateTime,
}

impl TryFrom<OtpCodeQueryResult> for OtpCode {
    type Error = anyhow::Error;

    fn try_from(value: OtpCodeQueryResult) -> anyhow::Result<OtpCode> {
        let otp_code = OtpCodeBuilder::default()
            .id(value.id)
            .user_id(value.user_id)
            .code(value.code)
            .expires_at(value.expires_at.and_utc())
            .date_created(value.date_created.and_utc())
            .date_updated(value.date_updated.and_utc())
            .build()?;

        Ok(otp_code)
    }
}

impl TryFrom<&OtpCodeQueryResult> for OtpCode {
    type Error = anyhow::Error;

    fn try_from(value: &OtpCodeQueryResult) -> anyhow::Result<OtpCode> {
        let otp_code = OtpCodeBuilder::default()
            .id(value.id)
            .user_id(value.user_id)
            .code(value.code.clone())
            .expires_at(value.expires_at.and_utc())
            .date_created(value.date_created.and_utc())
            .date_updated(value.date_updated.and_utc())
            .build()?;

        Ok(otp_code)
    }
}

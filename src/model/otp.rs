use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OtpCode {
    user_id: Uuid,
    code: String,
    active: bool,
    expires_at: NaiveDateTime,
}

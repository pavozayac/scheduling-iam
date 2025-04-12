use derive_builder::Builder;
use uuid::Uuid;

// Value object for OTP code
#[derive(Debug, Clone, Builder)]
pub struct OtpCode {
    id: Uuid,
    user_id: Uuid,
    code: String,
    expires_at: chrono::DateTime<chrono::Utc>,
    #[builder(default = "chrono::Utc::now()")]
    date_created: chrono::DateTime<chrono::Utc>,
    #[builder(default = "chrono::Utc::now()")]
    date_updated: chrono::DateTime<chrono::Utc>,
}

impl OtpCode {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn expires_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.expires_at
    }

    pub fn date_created(&self) -> chrono::DateTime<chrono::Utc> {
        self.date_created
    }

    pub fn date_updated(&self) -> chrono::DateTime<chrono::Utc> {
        self.date_updated
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.date_updated
    }
}

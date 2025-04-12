use derive_builder::Builder;
use uuid::Uuid;

// Value object for OTP code
#[derive(Debug, Clone, Builder)]
pub struct OtpCode {
    id: Uuid,
    user_id: Uuid,
    code: String,
    expires_at: time::UtcDateTime,
    #[builder(default = "time::UtcDateTime::now()")]
    date_created: time::UtcDateTime,
    #[builder(default = "time::UtcDateTime::now()")]
    date_updated: time::UtcDateTime,
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

    pub fn expires_at(&self) -> time::UtcDateTime {
        self.expires_at
    }

    pub fn date_created(&self) -> time::UtcDateTime {
        self.date_created
    }

    pub fn date_updated(&self) -> time::UtcDateTime {
        self.date_updated
    }

    pub fn is_expired(&self) -> bool {
        time::UtcDateTime::now() > self.date_updated
    }
}

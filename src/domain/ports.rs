pub mod repositories {
    use crate::domain::user::User;
    use uuid::Uuid;

    #[async_trait::async_trait]
    pub trait UserRepository {
        async fn find_by_id(&self, user_id: &Uuid) -> anyhow::Result<User>;
        async fn find_by_email(&self, email: &str) -> anyhow::Result<User>;
        async fn find_by_recovery_code(&self, code: &str) -> anyhow::Result<User>;
        async fn save(&self, user: &User) -> anyhow::Result<()>;
    }
}

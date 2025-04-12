use sqlx::Transaction;

use crate::{
    domain::{ports::repositories::UserRepository, user::User},
    infrastructure::database::{
        time_utils::utc_to_primitive_datetime,
        user_mapper::{
            OtpCodeQueryResult, PsqlUserMapper, RecoveryCodeQueryResult, UserQueryResult,
        },
    },
};

pub struct PsqlUserRepository {
    pool: sqlx::PgPool,
}

impl PsqlUserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn upsert_user(
        txn: &mut Transaction<'_, sqlx::Postgres>,
        user: &User,
    ) -> anyhow::Result<()> {
        let now_primitive = utc_to_primitive_datetime(time::UtcDateTime::now());

        sqlx::query!(
            "
            insert into users (id, email, registered, date_updated) values ($1, $2, $3, $4)
            on conflict (id)
            do update set
            email = excluded.email,
            registered = excluded.registered,
            date_updated = excluded.date_updated;
            ",
            user.user_id(),
            user.email(),
            user.registered(),
            now_primitive,
        )
        .execute(txn.as_mut())
        .await?;

        Ok(())
    }

    async fn update_otp(
        txn: &mut Transaction<'_, sqlx::Postgres>,
        user: &User,
    ) -> anyhow::Result<()> {
        let now_primitive = utc_to_primitive_datetime(time::UtcDateTime::now());

        sqlx::query!(
            "
            update otps
            set deleted = $2
            where user_id = $1
            ",
            user.user_id(),
            now_primitive,
        )
        .execute(txn.as_mut())
        .await?;

        if let Some(otp) = user.otp() {
            sqlx::query!(
                "
                insert into otps (id, user_id, code, expires_at, date_created, date_updated)
                values ($1, $2, $3, $4, $5, $6)
                ",
                otp.id(),
                otp.user_id(),
                otp.code(),
                utc_to_primitive_datetime(otp.expires_at()),
                utc_to_primitive_datetime(otp.date_created()),
                utc_to_primitive_datetime(otp.date_updated()),
            )
            .execute(txn.as_mut())
            .await?;
        }

        Ok(())
    }

    async fn update_recovery_codes(
        txn: &mut Transaction<'_, sqlx::Postgres>,
        user: &User,
    ) -> anyhow::Result<()> {
        let now_primitive = utc_to_primitive_datetime(time::UtcDateTime::now());

        sqlx::query!(
            "
            update recovery_codes
            set deleted = $2
            where user_id = $1
            ",
            user.user_id(),
            now_primitive,
        )
        .execute(txn.as_mut())
        .await?;

        for recovery_code in user.recovery_codes() {
            sqlx::query!(
                "
                insert into recovery_codes (user_id, code, date_created, date_updated)
                values ($1, $2, $3, $4)
                ",
                recovery_code.user_id(),
                recovery_code.code(),
                utc_to_primitive_datetime(recovery_code.date_created()),
                utc_to_primitive_datetime(recovery_code.date_updated()),
            )
            .execute(txn.as_mut())
            .await?;
        }

        Ok(())
    }

    async fn get_recovery_codes_from_user(
        txn: &mut Transaction<'_, sqlx::Postgres>,
        user_id: &uuid::Uuid,
    ) -> anyhow::Result<Vec<RecoveryCodeQueryResult>> {
        let recovery_codes = sqlx::query_as!(
            RecoveryCodeQueryResult,
            "
            select user_id, code, date_created, date_updated
            from recovery_codes
            where user_id = $1
            ",
            user_id,
        )
        .fetch_all(txn.as_mut())
        .await?;

        Ok(recovery_codes)
    }

    async fn get_otp_from_user(
        txn: &mut Transaction<'_, sqlx::Postgres>,
        user_id: &uuid::Uuid,
    ) -> anyhow::Result<Option<OtpCodeQueryResult>> {
        let otp = sqlx::query_as!(
            OtpCodeQueryResult,
            "
            select id, user_id, code, expires_at, date_created, date_updated
            from otps
            where user_id = $1
            ",
            user_id,
        )
        .fetch_optional(txn.as_mut())
        .await?;

        Ok(otp)
    }
}

#[async_trait::async_trait]
impl UserRepository for PsqlUserRepository {
    async fn save(&self, user: &User) -> anyhow::Result<()> {
        let mut txn = self.pool.begin().await?;

        Self::upsert_user(&mut txn, user).await?;
        Self::update_otp(&mut txn, user).await?;
        Self::update_recovery_codes(&mut txn, user).await?;

        txn.commit().await?;

        Ok(())
    }

    async fn find_by_id(&self, user_id: &uuid::Uuid) -> anyhow::Result<User> {
        let mut txn = self.pool.begin().await?;

        let user_query = sqlx::query_as!(
            UserQueryResult,
            "
            select id, email, registered, date_created, date_updated from users
            where id = $1
            ",
            user_id,
        )
        .fetch_one(txn.as_mut())
        .await?;

        let recovery_codes_query = Self::get_recovery_codes_from_user(&mut txn, user_id).await?;
        let otp_query = Self::get_otp_from_user(&mut txn, user_id).await?;

        txn.commit().await?;

        let user =
            PsqlUserMapper::map_user_query_result(user_query, otp_query, &recovery_codes_query)?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<User> {
        let mut txn = self.pool.begin().await?;

        let user_query = sqlx::query_as!(
            UserQueryResult,
            "
            select id, email, registered, date_created, date_updated from users
            where email = $1
            ",
            email,
        )
        .fetch_one(txn.as_mut())
        .await?;

        let recovery_codes_query =
            Self::get_recovery_codes_from_user(&mut txn, &user_query.id).await?;
        let otp_query = Self::get_otp_from_user(&mut txn, &user_query.id).await?;

        txn.commit().await?;

        let user =
            PsqlUserMapper::map_user_query_result(user_query, otp_query, &recovery_codes_query)?;

        Ok(user)
    }

    async fn find_by_recovery_code(&self, code: &str) -> anyhow::Result<User> {
        let mut txn = self.pool.begin().await?;

        let user_query = sqlx::query_as!(
            UserQueryResult,
            "
            select u.id, u.email, u.registered, u.date_created, u.date_updated
            from users u
            join recovery_codes rc on rc.user_id = u.id
            where rc.code = $1
            ",
            code,
        )
        .fetch_one(txn.as_mut())
        .await?;

        let recovery_codes_query =
            Self::get_recovery_codes_from_user(&mut txn, &user_query.id).await?;
        let otp_query = Self::get_otp_from_user(&mut txn, &user_query.id).await?;

        txn.commit().await?;

        let user =
            PsqlUserMapper::map_user_query_result(user_query, otp_query, &recovery_codes_query)?;

        Ok(user)
    }
}

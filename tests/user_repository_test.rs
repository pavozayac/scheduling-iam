use scheduling_iam::{
    domain::{ports::repositories::UserRepository, user::UserBuilder},
    infrastructure::database::user_repository::PsqlUserRepository,
};
mod common;

#[tokio::test]
async fn should_insert_user_into_db() {
    let pool = common::setup_migrated_postgres().await;

    let user_repo = PsqlUserRepository::new(pool.clone());

    let user_id: uuid::Uuid = uuid::Uuid::new_v4();

    let user = UserBuilder::default()
        .user_id(user_id)
        .email("testuser@example.com".to_string())
        .registered(true)
        .build()
        .unwrap();

    let user_save_result = user_repo.save(&user).await;
    assert!(user_save_result.is_ok(), "`{:?}`", user_save_result);

    let inserted_user = sqlx::query!(
        "
        select id, email, registered from users
        where id = $1;
        ",
        user.user_id()
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(user_id, inserted_user.id);
    assert_eq!("testuser@example.com", inserted_user.email);
    assert!(inserted_user.registered, "{:?}", inserted_user.registered);
}

#[tokio::test]
async fn should_find_user_by_id() {
    let pool = common::setup_migrated_postgres().await;
    let user_repo = PsqlUserRepository::new(pool.clone());

    let user_id: uuid::Uuid = uuid::Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, email) VALUES ($1, $2)",
        user_id,
        "findbyid@example.com"
    )
    .execute(&pool)
    .await
    .unwrap();

    let found_user = user_repo.find_by_id(&user_id).await.unwrap();
    assert_eq!(user_id, found_user.user_id());
    assert_eq!("findbyid@example.com", found_user.email());
}

#[tokio::test]
async fn should_find_user_by_email() {
    let pool = common::setup_migrated_postgres().await;
    let user_repo = PsqlUserRepository::new(pool.clone());

    let user_id: uuid::Uuid = uuid::Uuid::new_v4();
    let test_email = "findbyemail@example.com";

    sqlx::query!(
        "INSERT INTO users (id, email) VALUES ($1, $2)",
        user_id,
        test_email
    )
    .execute(&pool)
    .await
    .unwrap();

    let found_user = user_repo.find_by_email(test_email).await.unwrap();
    assert_eq!(user_id, found_user.user_id());
    assert_eq!(test_email, found_user.email());
}

#[tokio::test]
async fn should_find_user_by_recovery_code() {
    let pool = common::setup_migrated_postgres().await;
    let user_repo = PsqlUserRepository::new(pool.clone());

    let user_id: uuid::Uuid = uuid::Uuid::new_v4();
    let test_email = "findbyemail@example.com";
    let recovery_code = "testcode12345678";

    sqlx::query!(
        "INSERT INTO users (id, email) VALUES ($1, $2)",
        user_id,
        test_email
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO recovery_codes (user_id, code) VALUES ($1, $2)",
        user_id,
        recovery_code
    )
    .execute(&pool)
    .await
    .unwrap();

    let found_user = user_repo
        .find_by_recovery_code(recovery_code)
        .await
        .unwrap();

    assert_eq!(user_id, found_user.user_id());
    assert_eq!(test_email, found_user.email());
    assert_eq!(1, found_user.recovery_codes().len());
}

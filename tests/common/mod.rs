use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};

lazy_static! {
    static ref POSTGRES_CONTAINER: async_once::AsyncOnce<ContainerAsync<Postgres>> =
        async_once::AsyncOnce::new(init_postgres_container());
}

async fn init_postgres_container() -> ContainerAsync<Postgres> {
    Postgres::default().start().await.unwrap()
}

pub async fn setup_migrated_postgres() -> sqlx::postgres::PgPool {
    let port = POSTGRES_CONTAINER
        .get()
        .await
        .get_host_port_ipv4(5432)
        .await
        .unwrap();

    let options = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(5));

    let pool = options
        .connect_with(
            sqlx::postgres::PgConnectOptions::new()
                .database("postgres")
                .username("postgres")
                .password("postgres")
                .host("127.0.0.1")
                .port(port),
        )
        .await
        .unwrap();

    sqlx::migrate!("src/infrastructure/database/migrations")
        .run(&pool)
        .await
        .unwrap();

    pool
}

mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let pool = sqlx::postgres::PgPool::connect("postgres://postgres@localhost:5432/scheduling-iam")
        .await?;

    sqlx::migrate!("src/infrastructure/database/migrations")
        .run(&pool)
        .await?;

    Ok(())
}

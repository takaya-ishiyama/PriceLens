use sqlx::PgPool;
use uuid::Uuid;

use super::stb_data::stb_organization;

pub async fn setup_database(pool: &PgPool) {
    sqlx::migrate!("src/db/migrations").run(pool).await.unwrap();
    let stb = stb_organization()[0].get_params();

    let id = Uuid::parse_str(&stb.id).unwrap();

    sqlx::query!(
        r#"
        INSERT INTO organization (id, name, organization_type, created_at, updated_at)
        VALUES ($1,$2, 'PUBLIC', '2021-09-01 00:00:00', '2021-09-01 00:00:00')
        "#,
        id,
        stb.name
    )
    .execute(pool)
    .await;
}

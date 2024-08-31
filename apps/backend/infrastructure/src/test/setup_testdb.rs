use sqlx::PgPool;

use super::stb_data::stb_organization;

pub async fn setup_database(pool: &PgPool) {
    sqlx::migrate!("src/db/migrations").run(pool).await.unwrap();
    sqlx::query!(
        r#"
        INSERT INTO organization (id, name, organization_type, created_at, updated_at)
        VALUES ('17b5ac0c-1429-469a-8522-053f7bf0f80d','名無しの組織', 'PUBLIC', '2021-09-01 00:00:00', '2021-09-01 00:00:00')
        "#,
    )
    .execute(pool)
    .await;
}

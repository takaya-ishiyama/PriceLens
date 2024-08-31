use sqlx::PgPool;
use uuid::Uuid;

use crate::test::stb_data::stb_organization;

pub async fn test_seeder_organization(pool: &PgPool) {
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
    .await
    .unwrap();
}

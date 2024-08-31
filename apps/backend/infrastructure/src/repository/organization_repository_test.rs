#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use domain::{
        infrastructure::interface::repository::organization_repository_interface::OrganizationRepository,
        value_object::organaization::organization_type::ORGANIZATION_TYPE,
    };
    use sqlx::PgPool;

    use crate::{
        repository::organization_repository::OrganizationRepositoryImpl,
        test::{
            setup_testdb::setup_database, stb_data::stb_organization,
            test_seeder::organization::test_seeder_organization,
        },
    };

    #[sqlx::test]
    async fn test_create_true_response(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = OrganizationRepositoryImpl::new(db);

        let ognz_type = ORGANIZATION_TYPE::PUBLIC;
        let organization = repo.create("test", &ognz_type, Some("")).await.unwrap();

        assert_eq!(organization.get_params().name, "test".to_string());

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_one(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        test_seeder_organization(&pool).await;
        let stb = stb_organization();
        let db = Arc::new(pool);
        let repo = OrganizationRepositoryImpl::new(db);

        let organization = repo.find_one_by_id(&stb[0].get_params().id).await.unwrap();

        assert_eq!(organization.get_params().id, stb[0].get_params().id);

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_with_pagenate(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        test_seeder_organization(&pool).await;
        let stb = stb_organization();
        let db = Arc::new(pool);
        let repo = OrganizationRepositoryImpl::new(db);

        let organization = repo.find_one_by_id(&stb[0].get_params().id).await.unwrap();

        assert_eq!(organization.get_params().id, stb[0].get_params().id);

        Ok(())
    }
}

use async_graphql::{Context, Object, SimpleObject};
use domain::{
    infrastructure::interface::repository::repository_interface::Repositories,
    value_object::organaization::{organization::Organization, organization_type},
};
use usecase::organization::usecase::OrganizationInteractor;

use crate::{db::persistence::postgres::DB, repository::repository_impl::RepositoryImpls};

use super::schema::organization::OrganizationSchema;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_organization<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "name of object")] name: String,
        #[graphql(desc = "organization_type of object")] organization_type: String,
        #[graphql(desc = "private_key of object")] private_key: Option<String>,
    ) -> Result<OrganizationSchema, String> {
        let db = ctx.data::<DB>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);
        let ognz_usecase = OrganizationInteractor::new(&repo);

        let _organization_type = match organization_type.as_str() {
            "PUBLIC" => organization_type::ORGANIZATION_TYPE::PUBLIC,
            "PRIVATE" => organization_type::ORGANIZATION_TYPE::PRIVATE,
        };

        let _private_key: Option<&str> = private_key.as_ref().map(|s| s.as_str());

        let create_ognz = ognz_usecase
            .create_organization(&name, &_organization_type, _private_key)
            .await
            .unwrap();

        let ognz_params = create_ognz.get_params();
        Ok(OrganizationSchema::new(
            ognz_params.id,
            ognz_params.name,
            ognz_params.organization_type,
        ))
    }
}

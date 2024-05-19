use crate::{db::persistence::postgres::DB, repository::repository_impl::RepositoryImpls};
use async_graphql::*;
use domain::{
    infrastructure::interface::repository::repository_interface::Repositories,
    value_object::organaization::organization_type::ORGANIZATION_TYPE as DOMAIN_ORGANIZATION_TYPE,
};
use usecase::organization::usecase::OrganizationInteractor;

use super::schema::organization::{OrganizationSchema, ORGANIZATION_TYPE};

pub struct Query;

pub struct Token(pub String);

#[Object]
impl Query {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
    async fn organization_find_one<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "id of object")] id: ID,
    ) -> Result<OrganizationSchema, String> {
        let db = ctx.data::<DB>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);

        let organization_usecase = OrganizationInteractor::new(&repo);

        let organization = organization_usecase.find_one_by_id(&id).await.unwrap();

        let ognz_params = organization.get_params();
        let resp_ognz_type = match ognz_params.organization_type {
            DOMAIN_ORGANIZATION_TYPE::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
            DOMAIN_ORGANIZATION_TYPE::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
        };
        Ok(OrganizationSchema::new(
            ognz_params.id,
            ognz_params.name,
            resp_ognz_type,
        ))
    }
}

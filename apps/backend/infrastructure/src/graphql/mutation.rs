use async_graphql::{Context, Object};
use domain::{
    infrastructure::interface::repository::repository_interface::Repositories,
    value_object::{organaization::organization_type, Error::app_error::AppError},
};
use usecase::{item::usecase::ItemInteractor, organization::usecase::OrganizationInteractor};

use crate::{db::persistence::postgres::DB, repository::repository_impl::RepositoryImpls};

use super::schema::{
    item::ItemSchema,
    organization::{OrganizationSchema, ORGANIZATION_TYPE},
};

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_organization<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "name of object")] name: String,
        #[graphql(desc = "organization_type of object")] organization_type: String,
        #[graphql(desc = "private_key of object")] private_key: Option<String>,
    ) -> Result<OrganizationSchema, AppError> {
        let db = ctx.data::<DB>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);
        let ognz_usecase = OrganizationInteractor::new(&repo);

        let _private_key: Option<&str> = private_key.as_ref().map(|s| s.as_str());

        let _create_ognz = ognz_usecase
            .create_organization(&name, &organization_type, _private_key)
            .await;

        let create_ognz = match _create_ognz {
            Ok(ognz) => ognz,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()).into());
            }
        };

        let ognz_params = create_ognz.get_params();
        let resp_ognz_type = match ognz_params.organization_type {
            organization_type::ORGANIZATION_TYPE::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
            organization_type::ORGANIZATION_TYPE::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
        };
        Ok(OrganizationSchema::new(
            ognz_params.id,
            ognz_params.name,
            resp_ognz_type,
        ))
    }
    async fn create_item<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "item name")] name: String,
        #[graphql(desc = "item organization_id")] organization_id: String,
    ) -> Result<ItemSchema, AppError> {
        let db = ctx.data::<DB>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);
        let item_usecase = ItemInteractor::new(&repo);

        let created_item_result = item_usecase.create_item(&name, &organization_id).await;

        let created_item = match created_item_result {
            Ok(item) => item.get_params(),
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()).into());
            }
        };

        Ok(ItemSchema::new(
            created_item.id,
            created_item.name,
            created_item.organization_id,
        ))
    }
}

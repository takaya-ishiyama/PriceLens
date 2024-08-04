use crate::{db::persistence::postgres::DB, repository::repository_impl::RepositoryImpls};
use async_graphql::{
    connection::{Connection, Edge, EmptyFields},
    *,
};
use domain::{
    infrastructure::interface::repository::repository_interface::Repositories,
    value_object::{
        organaization::organization_type::ORGANIZATION_TYPE as DOMAIN_ORGANIZATION_TYPE,
        Error::app_error::AppError,
    },
};
use usecase::organization::usecase::OrganizationInteractor;

use super::schema::{
    organization::{OrganizationSchema, ORGANIZATION_TYPE},
    pagenate::CustomConnectionFields,
};

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
    ) -> Result<OrganizationSchema, AppError> {
        let db = ctx.data::<DB>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);

        let organization_usecase = OrganizationInteractor::new(&repo);

        let _organization = organization_usecase.find_one_by_id(&id).await;

        let organization = match _organization {
            Ok(organization) => organization,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()).into());
            }
        };

        Ok(OrganizationSchema::new(organization))
    }

    async fn organization_find_many_by_name<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "name")] name: String,
    ) -> Result<Vec<OrganizationSchema>, AppError> {
        let db = ctx.data::<DB>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);

        let organization_usecase = OrganizationInteractor::new(&repo);

        let _organizations = organization_usecase.find_many_by_name(&name).await;

        let organizations = match _organizations {
            Ok(organization) => organization,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()).into());
            }
        };

        let array_ognz_params = OrganizationSchema::new_from_domain_organizations(organizations);

        Ok(array_ognz_params)
    }

    async fn organization_find_all_with_pagenate<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "after")] after: Option<String>,
        #[graphql(desc = "before")] before: Option<String>,
        #[graphql(desc = "first")] first: Option<i32>,
        #[graphql(desc = "last")] last: Option<i32>,
    ) -> Result<Connection<String, Vec<OrganizationSchema>, EmptyFields, EmptyFields>, AppError>
    {
        let db = ctx.data::<DB>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);

        let organization_usecase = OrganizationInteractor::new(&repo);

        let _ogn = organization_usecase
            .find_all_with_pagenate(after, before, first, last)
            .await;

        let mut connection = Connection::new(true, true);

        let organization = match _ogn {
            Ok(organization) => organization,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()).into());
            }
        };

        let mut cursor = "none".to_string();
        let array_ognz = organization
            .iter()
            .map(|_org| {
                let params = _org.get_params();
                cursor = params.id.clone();
                let org_type = match params.organization_type {
                    DOMAIN_ORGANIZATION_TYPE::PUBLIC => ORGANIZATION_TYPE::PUBLIC,
                    DOMAIN_ORGANIZATION_TYPE::PRIVATE => ORGANIZATION_TYPE::PRIVATE,
                };
                OrganizationSchema::new(params.id, params.name, org_type)
            })
            .collect();

        connection.edges.push(Edge::new(cursor, array_ognz));

        Ok(connection)
    }
}

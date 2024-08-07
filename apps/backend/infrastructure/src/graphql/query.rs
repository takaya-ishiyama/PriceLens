use crate::{db::persistence::postgres::DB, repository::repository_impl::RepositoryImpls};
use async_graphql::{
    connection::{Connection, Edge, EmptyFields},
    *,
};
use domain::{
    infrastructure::interface::repository::repository_interface::Repositories,
    value_object::Error::app_error::AppError,
};
use usecase::organization::usecase::OrganizationInteractor;

use super::schema::organization::OrganizationSchema;

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

        let array_ognz_params = OrganizationSchema::new_from_vec(organizations);

        Ok(array_ognz_params)
    }

    async fn organization_find_all_with_pagenate<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "after")] after: Option<String>,
        #[graphql(desc = "before")] before: Option<String>,
        #[graphql(desc = "first")] first: Option<i32>,
        #[graphql(desc = "last")] last: Option<i32>,
    ) -> Result<Connection<String, OrganizationSchema, EmptyFields, EmptyFields>, AppError> {
        let db = ctx.data::<DB>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);

        let organization_usecase = OrganizationInteractor::new(&repo);

        let _ogn = organization_usecase
            .find_all_with_pagenate(after, before, first, last)
            .await;

        let organizations = match _ogn {
            Ok(organization) => organization,
            Err(e) => {
                return Err(anyhow::anyhow!(e.to_string()).into());
            }
        };

        let array_ognz = OrganizationSchema::new_from_vec(organizations);

        let mut connection = Connection::new(true, true);
        // let cursor = array_ognz[0].cursor();
        // for ognz in array_ognz {
        //     connection.edges.push(Edge::new(ognz.cursor(), ognz));
        // }
        // connection.edges.push(Edge::new(cursor, array_ognz));
        connection.edges.extend(
            array_ognz
                .into_iter()
                .map(|ognz| Edge::with_additional_fields(ognz.cursor(), ognz, EmptyFields)),
        );

        Ok(connection)
    }
}

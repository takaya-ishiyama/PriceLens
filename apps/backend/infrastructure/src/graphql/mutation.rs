use async_graphql::{Context, Object, SimpleObject};
use domain::infrastructure::interface::repository::repository_interface::Repositories;

use crate::{db::persistence::postgres::DB, repository::repository_impl::RepositoryImpls};

pub struct Mutation;

// #[Object]
// impl Mutation {}

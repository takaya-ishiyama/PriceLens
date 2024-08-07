use domain::{
    infrastructure::interface::repository::{
        organization_repository_interface::OrganizationRepository,
        repository_interface::Repositories,
    },
    value_object::{
        organaization::{organization::Organization, organization_type::ORGANIZATION_TYPE},
        page_info::PageInfo,
    },
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct OrganizationInteractor<'r, R: Repositories> {
    organization_repo: &'r R::OrganizationRepo,
}

impl<'r, R: Repositories> OrganizationInteractor<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            organization_repo: repositories.organization_repo(),
        }
    }

    pub async fn create_organization(
        &self,
        name: &str,
        organization_type: &str,
        private_key: Option<&'r str>,
    ) -> Result<Organization, String> {
        let exist_same_name = self.organization_repo.exist_same_name(name).await?;
        match exist_same_name {
            true => return Err("Organization name already exists".to_string()),
            false => (),
        };

        let _organization_type = match organization_type {
            "PUBLIC" => ORGANIZATION_TYPE::PUBLIC,
            "PRIVATE" => ORGANIZATION_TYPE::PRIVATE,
            _ => return Err("Invalid organization type".to_string()),
        };
        let organization = self
            .organization_repo
            .create(name, &_organization_type, private_key)
            .await?;
        Ok(organization)
    }

    pub async fn find_one_by_id(&self, id: &str) -> Result<Organization, String> {
        let organization = self.organization_repo.find_one_by_id(id).await?;
        Ok(organization)
    }

    pub async fn find_many_by_name(&self, name: &str) -> Result<Vec<Organization>, String> {
        let organizations = self.organization_repo.find_many_by_name(name).await?;
        Ok(organizations)
    }

    pub async fn find_all_with_pagenate(
        &self,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Vec<Organization>, String> {
        let organizations = self
            .organization_repo
            .find_all_with_pagenate(after, before, first, last)
            .await?;
        Ok(organizations)
    }
}

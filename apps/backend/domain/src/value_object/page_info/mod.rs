pub struct PageInfo {
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
}

pub struct GetParams {
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

impl PageInfo {
    pub fn new(
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Self {
        Self {
            after,
            before,
            first,
            last,
        }
    }

    pub fn get_params(&self) -> GetParams {
        GetParams {
            after: self.after.clone(),
            before: self.before.clone(),
            first: self.first,
            last: self.last,
        }
    }

    pub fn format_api_resp(
        &self,
    ) -> Connection<String, Vec<OrganizationSchema>, EmptyFields, EmptyFields> {
    }
}

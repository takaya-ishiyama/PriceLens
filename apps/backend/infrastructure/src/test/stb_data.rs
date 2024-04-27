use domain::value_object::organaization::{
    organization::Organization, organization_type::ORGANIZATION_TYPE,
};

pub fn stb_organization() -> Vec<Organization> {
    let orgn1 = Organization::new(
        "17b5ac0c-1429-469a-8522-053f7bf0f80d",
        "名無しの組織",
        &ORGANIZATION_TYPE::PRIVATE,
        "",
        "",
        "",
    );
    Vec::from([orgn1])
}

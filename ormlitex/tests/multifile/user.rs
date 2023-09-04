use ormlitex::types::Uuid;
use ormlitex::model::*;
use crate::organization::Organization;

#[derive(Debug, Model)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub age: u8,
    #[ormlitex(join_column = "organization_id")]
    pub organization: Join<Organization>,
}
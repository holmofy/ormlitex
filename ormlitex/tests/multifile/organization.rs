use ormlitex::types::Uuid;
use ormlitex::model::*;

#[derive(Debug, Model)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
}
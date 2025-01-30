use serde::{Deserialize, Serialize};
//
//list
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions{
    pub page: Option<usize>,
    pub limit: Option<usize>
}

// create
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema{
    pub name: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
}
//
#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateUserSchema{
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_verified: Option<bool>,
}
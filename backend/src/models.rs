

use serde::{Serialize, Deserialize};
use diesel::prelude::*;


#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub slug: String,
    pub password:String,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateUserResponse {
    pub token: String,
}

#[derive(Queryable, Debug)]
pub struct UserDTO {
    pub id: i32,
    pub email: String,
    pub slug: String,
    pub balance: f64
}



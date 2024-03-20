use std::arch::aarch64::float32x2_t;
use serde::{Serialize, Deserialize};
use diesel::prelude::Queryable;

#[derive(Debug, Serialize, Deserialize,Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub slug: String,
    pub balance: float32x2_t
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
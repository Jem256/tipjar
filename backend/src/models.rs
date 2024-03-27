
use serde::{Serialize, Deserialize};
use diesel::prelude::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub slug: String,
    pub password:String,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize,Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct LoggedInUser{
    pub id:i32,
    pub email: String,
    pub name: String,
    pub slug: String,
    pub balance:String
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq,Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub slug:String,
    pub balance:String
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterUserRequest {
    pub email: String,
    pub name: String,
    pub password: String,

}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserResponse{
    pub email: String,
    pub name: String,
    pub slug:String,
    pub balance: String
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentDetails{
    pub slug:String,
    pub amount_in_satoshi:i32,

}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq,Insertable,Selectable)]
#[diesel(table_name = crate::schema::user_transactions)]
pub struct InvoiceDetails{
    pub payment_request:String,
    pub amount_in_satoshi:i32,
    pub payment_addr:String,
    pub user_id:i32,
    pub status:i32
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq,Insertable,Selectable)]
#[diesel(table_name = crate::schema::user_transactions)]
pub struct UserTransaction{
    pub id:i32,
    pub user_id:i32,
    pub amount_in_satoshi:i32,
    pub payment_request:String,
    pub payment_addr:String,
    pub status:i32
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



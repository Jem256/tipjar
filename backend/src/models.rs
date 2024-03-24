use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::sql_types::{Decimal, Float, Nullable, Numeric};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub slug: String,
    pub password:String

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvoiceDetails{
    pub payment_request:String,
    pub amount_in_satoshi:i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateUserResponse {
    pub token: String,
}
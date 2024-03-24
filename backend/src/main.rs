#[macro_use] extern crate rocket;
mod lnd;
use self::models::*;
mod database;
mod models;
mod schema;


use diesel::prelude::*;
use rocket::{get, launch, post, routes};
use rocket::fs::{FileServer, Options, relative};
use rocket::response::status::Created;
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use self::models::*;
use schema::users;

use crate::lnd::connect;
// use crate::schema::users;
// use serde::{Serialize, Deserialize};
// use rocket::serde::json::Json;
// use rocket::contrib::json::Json;



#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct UserForm<'a> {
    email: Option<&'a str>,
    password: Option<&'a str>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,
            register,
            login,
            get_user_by_slug,
            get_all_users
        ])
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rocket application!"
}

/*
* This function will register our user
 */
#[post("/register")]
// pub fn register(user_data: Json<CreateUserRequest>) -> Result<Created<String>, Box<dyn std::error::Error>> {
//     use schema::users;
    
//     let user: CreateUserRequest = user_data.into_inner();
//     let connection: &mut PgConnection = &mut database::establish_connection();

//     diesel::insert_into(users::table)
//         .values(user)
//         .execute(connection)?;

//     Ok(Created::new("/").body("User registered successfully".to_string()))
// }

pub fn register(user_data: Json<UserForm>) -> Result<(), Box<dyn std::error::Error>> {
    use schema::users::dsl::*;

    let user_form = serde_json::from_str::<UserForm>(user_data)?;

    diesel::insert_into(users::table).values(&user_form).execute(conn)?;

    Ok(())
}


/*c
* This function will login our user
* upon login the user will redirect to user/dashboard
 */
#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<AuthenticateUserRequest>) -> Result<Json<AuthenticateUserResponse>, Box<dyn std::error::Error>> {
    use schema::users::dsl::*;

    let credentials = credentials.into_inner();
    let connection = &mut database::establish_connection();

    // Fetch user by email to verify the password
    let user = users.filter(email.eq(&credentials.email))
        .first::<User>(&connection)?;

    // TODO: Implement password verification and token generation logic here

    // For demonstration purposes, returning a dummy token
    let dummy_token = "dummy_token".to_string();
    Ok(Json(AuthenticateUserResponse { token: dummy_token }))
}

/*
* This function create an endpoint when then userendpoint is visited
* Function will return and
 */
#[get("/user/<slug>")]
async fn get_user_by_slug(slug: String) -> Result<Json<UserDTO>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let connection: PgConnection = database::establish_connection();

    let user_data = users.select((id, email, slug, balance.nullable()))
        .filter(slug.eq(&slug))
        .first::<(i32, String, String, Option<diesel::sql_types::Numeric>)>(&mut connection)?;

    let user_dto = UserDTO {
        id: user_data.0,
        email: user_data.1,
        slug: user_data.2,
        balance: user_data.3,
    };

    Ok(Json(user_dto))
}


#[get("/users")]
async fn get_all_users() -> Result<Json<Vec<UserDTO>>, String> {
  use crate::schema::users::dsl::*;

  let connection: PgConnection = database::establish_connection();

  match users.load::<UserDTO>(&mut connection) {
    Ok(all_users) => Ok(Json(all_users)),
    Err(err) => Err(format!("Error fetching users: {}", err)),
  }
}




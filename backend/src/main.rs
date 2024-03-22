mod lnd;

use diesel::prelude::*;
use rocket::{get, launch, post, routes};
use rocket::fs::{FileServer, Options, relative};
use rocket::response::status::Created;
use crate::lnd::connect;
use crate::schema::users;
use rocket::serde::json::Json;

use self::models::*;
mod database;
mod schema;
mod models;

#[launch]
fn rocket() -> _ {
    rocket::build()

        // serve content from disk
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        // register routes
        .mount("/", routes![index])
}

#[get("/")]
fn index()  {
    use schema::users;

    let connection = &mut database::establish_connection();

    match users::table.select(users::all_columns).load::<User>( connection) {
        Ok(mut users) => Ok(users),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
/*
* This function will register our user
 */
#[post("/register")]
pub fn register(user: Json<CreateUserRequest>) -> Created<String> {
    use domain::schema::users;

    let user = user.into_inner();
    let connection = &mut database::establish_connection();
    match diesel::insert_into(users::table).values(&user).get_result::<Post>(connection) {
        Ok(post) => {
            let response = Response { body: ResponseBody::User(user) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
/*c
* This function will login our user
* upon login the user will redirect to user/dashboard
 */
#[post("/login")]
pub fn login(){


}

/*
* This function create an endpoint when then userendpoint is visited
* Function will return and
 */
#[get("/user/slug")]
pub fn get_user(){


}





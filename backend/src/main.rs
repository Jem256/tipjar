mod lnd;


use rocket::{get, launch, routes};
use rocket::fs::{FileServer, Options, relative};
use rocket_dyn_templates::{context, Template};
use self::schema::users::dsl::*;
use rocket::serde::json::Json;
use self::models::*;
mod database;
mod schema;
mod models;

#[launch]
fn rocket() -> _ {
    rocket::build()
        // add templating system
        .attach(Template::fairing())
        // serve content from disk
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        // register routes
        .mount("/", routes![index,hello,login,register])
}

#[get("/")]
fn index() -> Json<Vec<User>> {
    let connection = &mut database::establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");
}
#[get("/register")]
async fn register() -> Template {
    Json("register");
}
#[get("/login")]
async fn login() -> Template {
    Json("login");
}


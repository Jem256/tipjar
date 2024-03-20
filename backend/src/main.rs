use rocket::{get, launch, routes};
use rocket::fs::{FileServer, Options, relative};
use rocket_dyn_templates::{context, Template};
use rocket::request::FlashMessage;


#[launch]
fn rocket() -> _ {
    rocket::build()
        // add templating system
        .attach(Template::fairing())
        // serve content from disk
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        // register routes
        .mount("/", routes![root,hello,login,register])
}

#[get("/")]
async fn root() -> Template {
    Template::render("welcome", context! { message: "Welcome to TipJar"})
}
#[get("/register")]
async fn register() -> Template {
    Template::render("register", context! { message: "Register"})
}
#[get("/login")]
async fn login() -> Template {
    Template::render("login", context! { message: "Register"})
}

#[get("/hi?<name>")]
async fn hello(name: String, flash: Option<FlashMessage<'_>>) -> Template {
    let message = flash.map_or_else(|| String::default(), |msg| msg.message().to_string());
    Template::render("hello", context! { name , message })
}
use rocket::{get, launch, routes};
use rocket::fs::{FileServer, Options, relative};
use rocket_dyn_templates::{context, Template};
use rocket::request::FlashMessage;
use qrcode_generator::QrCodeEcc;



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
    qrcode_generator::to_png_to_file("Hello world!", QrCodeEcc::Low, 1024, "public/data/file_output.png").unwrap();

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
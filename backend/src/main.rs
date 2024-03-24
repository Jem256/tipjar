mod lnd;

use diesel::dsl::host;
use diesel::prelude::*;
use rocket::{get, launch, post, routes};
use rocket::fs::{FileServer, Options, relative};
use rocket::response::status::Created;
use crate::lnd::connect;
use crate::schema::users::dsl::*;
use rocket::serde::json::Json;
use crate::models::RegisterUserRequest;

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
        .mount("/", routes![register,login,generate_invoice])
}

// #[get("/")]
// fn index() ->  Vec<User>{
//     use crate::schema::users;
//     let connection = &mut establish_connection();
//     let students = users.load::<User>(connection)
//     .expect("Error loading students");
//     Json(students)
// }
/*
* This function will register our user
 */
#[post("/register",format = "json", data = "<new_user>")]
 pub fn register(new_user: Json<RegisterUserRequest>) -> Json<String> {

    use crate::schema::users;

    let connection = &mut database::establish_connection();
    let unique_slug = hex::encode(new_user.email.clone());
    let user = CreateUserRequest{
        email:  new_user.email.clone(),
        name:  new_user.name.clone(),
        password:new_user.password.clone(),
        slug:unique_slug,
        balance:"0".to_string()
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(connection)
        .expect("Error saving new user");

    let success="success".to_string();
    Json(success)
}
/*c
* This function will login our user
* upon login the user will redirect to dashboard
* will return logged user with the user details, email,balance, unique url
 */
#[post("/login",format = "json", data = "<login_details>")]
pub fn login(login_details: Json<LoginRequest>)->Json<UserResponse>{
    use crate::schema::users;
    let connection = &mut database::establish_connection();
    let sent_email= login_details.email.clone();
    let _pass= login_details.password.clone();


    let user_result = users::table
        .filter(users::email.eq(sent_email))
        .select(LoggedInUser::as_select())
        .first(connection);
    match user_result {
        Ok(user) => {
            // let id: i32 = user.id;
            // let email: String = user.email;
            // let slug: String = user.slug;
            // let balance: String = user.balance;

            //

            let user_response =UserResponse{
                name:user.name,
                email:user.email,
                slug:user.slug,
                balance:user.balance
            };

            Json(user_response)

            // Now you can use id, email, slug, and balance variables
        }

        _ => {
             let res=UserResponse{name:"".parse().unwrap(),email:"".parse().unwrap(),slug:"".parse().unwrap(), balance:"".parse().unwrap() };
            Json(res)
        }
    }



}

/*
* This function create an endpoint when then userendpoint is visited
* Function will return and

 */



#[post("/generate-invoice/<req_slug>",format = "json", data = "<payment_details>")]
pub async fn generate_invoice(req_slug:String, payment_details: Json<PaymentDetails>) ->Json<InvoiceDetails>{

    use crate::schema::users;
    let connection = &mut database::establish_connection();
    let user_result = users::table
        .filter(users::slug.eq(req_slug))
        .select(LoggedInUser::as_select())
        .first(connection);
    match user_result {
        Ok(user) => {
            //print!("{:?}", user);
            let payment_request = lnd::connect(payment_details.amount_in_satoshi).await;

            //save the payment request and the amount in a user transactions table
            //payment_request,amount and status,user_id,slug
            let invoice_details=InvoiceDetails{
                amount_in_satoshi:payment_details.amount_in_satoshi,
                payment_request
            };
            Json(invoice_details)


            // Now you can use id, email, slug, and balance variables
        }

        _ => {
            let res=InvoiceDetails{payment_request:"".parse().unwrap(),amount_in_satoshi:0 };
            Json(res)
        }
    }


}





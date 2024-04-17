#[macro_use] 
extern crate rocket;
extern crate rocket_cors;
mod lnd;

use diesel::prelude::*;
use rocket::{get, launch, post, routes};
use crate::schema::users::dsl::*;
use rocket::serde::json::Json;
use crate::invoice::invoice_look_up;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

use self::models::*;

extern crate cron_job;
use cron_job::CronJob;

mod database;
mod schema;
mod models;

mod invoice;

mod lnd;

pub struct Cors;

pub struct Cron;


use rocket::http::Method;

use rocket_cors::{
    AllowedOrigins, AllowedHeaders, Cors, CorsOptions
};

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://127.0.0.1:3000"
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(), 
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type", 
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Headers",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[launch]
fn rocket() -> _ {      
    rocket::build()
        // serve content from disk
        //.mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        // register routes
        .mount("/", routes![register,login,generate_invoice,fetch_user]).attach(make_cors())
}

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
pub async fn login(login_details: Json<LoginRequest>) ->Json<UserResponse>{
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
pub async fn generate_invoice(req_slug:String, payment_details: Json<PaymentDetails>) ->Json<InvoiceDetails> {
    use crate::schema::users;
    use crate::schema::user_transactions;
    let connection = &mut database::establish_connection();
    let user_result = users::table
        .filter(users::slug.eq(req_slug))
        .select(LoggedInUser::as_select())
        .first(connection);
    match user_result {
        Ok(user) => {

            let invoice_response = invoice::create_invoice(payment_details.amount_in_satoshi).await;

            let payment_addr_string=base64::encode(invoice_response.payment_addr);

            //let payment_add=base64::decode(payment_addr_string.clone()).unwrap();

            //println!("{:?}", payment_add);
            //save the payment request and the amount in a user transactions table
            //payment_request,amount and status,user_id,slug

            let invoice_details = InvoiceDetails {
                amount_in_satoshi: payment_details.amount_in_satoshi,
                payment_request:invoice_response.payment_request,
                payment_addr:payment_addr_string,
                user_id:user.id,
                status:0
            };
            diesel::insert_into(user_transactions::table)
                .values(&invoice_details)
                .execute(connection)
                .expect("Error saving invoice");
            Json(invoice_details)
            // Now you can use id, email, slug, and balance variables
        }

        _ => {
            let res = InvoiceDetails { payment_request: "".parse().unwrap(), amount_in_satoshi: 0,payment_addr:"".to_string(),user_id:0,status:0 };
            Json(res)
        }
    }
}


pub async fn refresh_invoice(incoming_user_id:i32){
    use self::schema::user_transactions::dsl::*;
    use crate::schema::users;
    let connection = &mut database::establish_connection();
    let user = users::table
        .find(&incoming_user_id)
        .select(LoggedInUser::as_select())
        .first(connection);

    let all_user_transactions = user_transactions
        .filter(user_id.eq(incoming_user_id))
        .filter(status.eq(0))
        .select(UserTransaction::as_select())
        .load(connection);
    //println!("{:?}", all_user_transactions);
    let mut new_balance  = 0;
    match all_user_transactions {
        Ok(transactions) => {
            for transaction in transactions {
                // Do something with each transaction here
                let payment_add=base64::decode(transaction.payment_addr).unwrap();
                let invoice_lookup = invoice::invoice_look_up(payment_add).await;
               if invoice_lookup.status > 0 {
                   new_balance += transaction.amount_in_satoshi;
                   diesel::update(user_transactions.find(transaction.id))
                       .set(status.eq(invoice_lookup.status))
                       .returning(UserTransaction::as_returning())
                       .get_result(connection).expect("Update failed");
               }


            }
            let user_balance= &user.unwrap().balance;
            let usr_bal=user_balance.parse::<i32>().unwrap();
            let total_balance =usr_bal + new_balance;

            diesel::update(users.find(incoming_user_id))
                .set(balance.eq(total_balance.to_string()))
                .returning(LoggedInUser::as_returning())
                .get_result(connection).expect("balance update failed");


        }
        Err(err) => {
            eprintln!("Error loading user transactions: {:?}", err);
        }
    }
}
#[get("/fetch-user/<request_id>")]
pub async fn fetch_user(request_id: i32) ->Json<UserResponse>{
    refresh_invoice(request_id.clone()).await;
    let connection = &mut database::establish_connection();

    let user_res = crate::schema::users::table
        .find(request_id)
        .select(LoggedInUser::as_select())
        .first(connection);

    match user_res {
        Ok(user) => {
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




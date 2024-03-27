#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::{get, launch, post, routes};
use crate::schema::users::dsl::*;
use rocket::serde::json::Json;
use crate::invoices::invoice_look_up;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

use self::models::*;

extern crate cron_job;
use cron_job::CronJob;

mod database;
mod schema;
mod models;

mod invoices;

mod lnd;

pub struct Cors;
pub struct Cron;


#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        // serve content from disk
        //.mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        // register routes
        .attach(Cors)
        .mount("/", routes![register,login,generate_invoice,refresh_invoice])
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
            //print!("{:?}", user);
            let invoice_response = lnd::connect(payment_details.amount_in_satoshi).await;
            //println!("{:?}", invoice_response.payment_addr);
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

#[get("/refresh/<incoming_user_id>")]
pub async fn refresh_invoice(incoming_user_id:i32){
    use self::schema::user_transactions::dsl::*;
    use crate::schema::user_transactions;
    use crate::schema::users;
    let connection = &mut database::establish_connection();
    // let user = users::table
    //     .find(&incoming_user_id)
    //     .select(LoggedInUser::as_select())
    //     .first(connection);
   // user_transactions.load::<UserTransaction>(connection).map(Json).expect("Error loading birds");

    let all_user_transactions = user_transactions
        .filter(user_id.eq(incoming_user_id))
        .filter(status.eq(0))
        .select(UserTransaction::as_select())
        .load(connection);
    //println!("{:?}", all_user_transactions);
    //let user_balance  = 0;
    match all_user_transactions {
        Ok(transactions) => {
            for transaction in transactions {
                // Do something with each transaction here
                let payment_add=base64::decode(transaction.payment_addr).unwrap();
                let invoice_lookup =invoices::invoice_look_up(payment_add).await;
               if invoice_lookup.status > 0 {
                   //balance + transaction.amount
                   diesel::update(user_transactions.find(transaction.id))
                       .set(status.eq(invoice_lookup.status))
                       .returning(UserTransaction::as_returning())
                       .get_result(connection).expect("Update failed");
               }


            }
        }
        Err(err) => {
            eprintln!("Error loading user transactions: {:?}", err);
        }
    }


    //for transaction in all_user_transactions {
        //println!("{:?}", transaction)
        // let payment_add=base64::decode(transaction.payment_addr);
        // let invoice_lookup =invoices::invoice_look_up(payment_add).await;
        //
        // diesel::update(user_transactions.find(transaction.id))
        //     .set(status.eq(invoice_lookup.status))
        //     .returning(InvoiceDetails::as_returning())
        //     .get_result(connection)
        //     .unwrap();
    //}
}





use rand::distributions::{Alphanumeric, DistString};
use tonic_lnd;
use sha2::{Sha256, Digest};
use std::env;
use dotenvy::dotenv;


pub struct InvoiceResponse{

    pub payment_addr:Vec<u8>,
    pub payment_request:String
}

pub async  fn connect(amount: i32)-> InvoiceResponse{
    dotenv().ok();

    let address= env::var("ADDRESS").expect("ADDRESS must be set");
    let cert_file_path= env::var("CERT_FILE_PATH").expect("CERT_FILE_PATH must be set");
    let macaroon_file_path= env::var("MACAROON_FILE_PATH").expect("MACAROON_FILE_PATH must be set");

    let cert_path = cert_file_path.to_string();
    let macaroon_path = macaroon_file_path.to_string();
    let address_path= address.to_string();
    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = tonic_lnd::connect(address_path, cert_path, macaroon_path)
        .await
        .expect("failed to connect");


    let hex_preimage = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    //let hex_preimage = "54686520717569636b2062726f776e19";


    let payment_description = b"descriptionas";
    let mut hasher = Sha256::new();
    hasher.update(payment_description);
    let payment_hash=hasher.finalize().to_vec();
    //println!("{:?}", payment_hash);
    // Compute the SHA-256 hash of the payment description

    let  invoice= client.lightning().add_invoice(tonic_lnd::lnrpc::Invoice{
        memo:"sass".to_string(),
        r_preimage: Vec::from(hex_preimage),
        r_hash: vec![],
        value: amount as i64,
        value_msat: 0,
        settled: false,
        creation_date: 0,
        settle_date: 0,
        payment_request:"".to_string(),
        description_hash: payment_hash,
        expiry: 0,
        fallback_addr: "".to_string(),
        cltv_expiry: 0,
        route_hints: vec![],
        private: false,
        add_index:1,
        settle_index: 0,
        amt_paid: 0,
        amt_paid_sat: 0,
        amt_paid_msat: 0,
        state: 0,
        htlcs: vec![],
        features: Default::default(),
        is_keysend: false,
        payment_addr: vec![],
        is_amp: false,
        amp_invoice_state: Default::default(),
    }).await.expect("failed to get info");
    let invoice_inner =invoice.into_inner();
   let invoice_response= InvoiceResponse{
        payment_addr:invoice_inner.payment_addr.clone(),
        payment_request:invoice_inner.payment_request.clone()
    };

    invoice_response


}



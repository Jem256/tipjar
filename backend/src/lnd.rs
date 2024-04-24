use rand::distributions::{Alphanumeric, DistString};
use tonic_lnd;
use sha2::{Sha256, Digest};

use tonic_lnd::Client;
use std::env;
use dotenvy::dotenv;

pub async  fn connect()-> Result<Client, Err>{
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

    Ok(client)

}



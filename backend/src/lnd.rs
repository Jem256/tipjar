use rand::distributions::{Alphanumeric, DistString};
use tonic_lnd;
use sha2::{Sha256, Digest};
use tonic_lnd::Client;




pub async  fn connect()-> Client{


    let address="https://127.0.0.1:10001";
    //let cert_file_path ="/Users/jose/.polar/networks/1/volumes/lnd/alice/tls.cert";

    let cert_file_path="/Users/jose/.polar/networks/2/volumes/lnd/alice/tls.cert";
    let macaroon_file_path="/Users/jose/.polar/networks/2/volumes/lnd/alice/data/chain/bitcoin/regtest/admin.macaroon";

    let cert_path = cert_file_path.to_string();
    let macaroon_path = macaroon_file_path.to_string();
    let address_path= address.to_string();
    // Connecting to LND requires only address, cert file, and macaroon file
    let mut client = tonic_lnd::connect(address_path, cert_path, macaroon_path)
        .await
        .expect("failed to connect");

    client

}



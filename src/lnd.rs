
use tonic_lnd;
pub async  fn connect(){
    let address="";
    let cert_file ="/Users/jose/.polar/networks/1/volumes/lnd/alice/tls.cert";
    let macaroon_file="/Users/jose/.polar/networks/1/volumes/lnd/alice/data/chain/bitcoin/regtest/admin.macaroon";

     // Connecting to LND requires only address, cert file, and macaroon file
     let mut client = tonic_lnd::connect(address, cert_file, macaroon_file)
         .await
         .expect("failed to connect");

     let info = client
         .lightning()
         // All calls require at least empty parameter
         .get_info(tonic_lnd::lnrpc::GetInfoRequest {})
         .await
         .expect("failed to get info");

    println!("{:#?}", info);

 }
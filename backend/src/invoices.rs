use rocket::futures::task::Spawn;
use rocket::serde::json::Json;
use tonic_lnd;
use sha2::{Sha256, Digest};
use tonic_lnd::invoicesrpc::lookup_invoice_msg::InvoiceRef;
use tonic_lnd::invoicesrpc::lookup_invoice_msg::InvoiceRef::PaymentAddr;
use tonic_lnd::invoicesrpc::LookupInvoiceMsg;
use tonic_lnd::lnrpc::{Amount, Invoice};
use tonic_lnd::lnrpc::invoice::InvoiceState;
use crate::models::InvoiceDetails;
use crate::schema::user_transactions::status;

pub struct InvoiceStatus{

    status:i32,
}
pub async  fn invoice_look_up(payment_addr: Vec<u8>)-> InvoiceStatus{


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

    let invoice_lookup=client.invoices()
        .lookup_invoice_v2(LookupInvoiceMsg{ lookup_modifier: 1, invoice_ref: Option::from(InvoiceRef::PaymentAddr(payment_addr)) })
        .await.expect("failed to get info");

    let invoice_state =InvoiceStatus{status:invoice_lookup.into_inner().state};
    invoice_state


}



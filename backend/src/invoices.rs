
use tonic_lnd;
use tonic_lnd::invoicesrpc::lookup_invoice_msg::InvoiceRef;
use tonic_lnd::invoicesrpc::LookupInvoiceMsg;
use std::env;
use dotenvy::dotenv;
#[derive(Debug)]
pub struct InvoiceStatus{

    pub status:i32,
}
pub async  fn invoice_look_up(payment_addr: Vec<u8>)-> InvoiceStatus{
    dotenv().ok();

    //let payment_address= payment_addr.as_bytes().to_vec();
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

    let invoice_lookup=client.invoices()
        .lookup_invoice_v2(LookupInvoiceMsg{ lookup_modifier: 1, invoice_ref: Option::from(InvoiceRef::PaymentAddr(payment_addr)) })
        .await.expect("failed to get info");

    let invoice_state =InvoiceStatus{status:invoice_lookup.into_inner().state};
    invoice_state

}



use rand::distributions::Alphanumeric;
use sha2::Sha256;
use tonic_lnd;
use tonic_lnd::invoicesrpc::lookup_invoice_msg::InvoiceRef;
use tonic_lnd::invoicesrpc::LookupInvoiceMsg;
use crate::lnd::{connect};
#[derive(Debug)]
pub struct InvoiceStatus{

    pub status:i32,
}
pub struct InvoiceResponse{

    pub payment_addr:Vec<u8>,
    pub payment_request:String
}
pub async  fn create_invoice(amount: i32){
    let client = connect();
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
pub async  fn invoice_look_up(payment_addr: Vec<u8>)-> InvoiceStatus{
    //connect to the node 
    let client = connect();

    let invoice_lookup=client.invoices()
        .lookup_invoice_v2(LookupInvoiceMsg{ lookup_modifier: 1, invoice_ref: Option::from(InvoiceRef::PaymentAddr(payment_addr)) })
        .await.expect("failed to get info");

    let invoice_state =InvoiceStatus{status:invoice_lookup.into_inner().state};
    invoice_state

}



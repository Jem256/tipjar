// @generated automatically by Diesel CLI.

diesel::table! {
    user_transactions (id) {
        id -> Int4,
        user_id -> Int4,
        amount_in_satoshi -> Int4,
        #[max_length = 255]
        payment_request -> Varchar,
        #[max_length = 500]
        payment_addr -> Varchar,
        status -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        #[max_length = 255]
        balance -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user_transactions,
    users,
);

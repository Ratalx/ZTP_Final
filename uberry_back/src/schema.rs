table! {
    favourites (login, fruit_id) {
        login -> Varchar,
        fruit_id -> Int4,
    }
}

table! {
    fruits (id) {
        id -> Int4,
        name -> Varchar,
        in_stock -> Bool,
    }
}

table! {
    orders (id) {
        id -> Int4,
        web_user_login -> Varchar,
        fruit_id -> Int4,
        order_date -> Timestamp,
    }
}

table! {
    users (login) {
        login -> Varchar,
        password -> Varchar,
        admin_user -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    favourites,
    fruits,
    orders,
    users,
);

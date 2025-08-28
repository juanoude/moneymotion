// @generated automatically by Diesel CLI.

diesel::table! {
    incomes (id) {
        id -> Integer,
        value -> Float,
        date -> Text,
        name -> Text,
        category -> Text,
    }
}

diesel::table! {
    spendings (id) {
        id -> Integer,
        value -> Float,
        date -> Text,
        name -> Text,
        category -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(incomes, spendings,);

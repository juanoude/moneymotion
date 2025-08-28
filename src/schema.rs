// @generated automatically by Diesel CLI.

diesel::table! {
    spendings (id) {
        id -> Integer,
        value -> Float,
        date -> Text,
        name -> Text,
        category -> Text,
    }
}

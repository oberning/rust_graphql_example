// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id) {
        id -> Nullable<Integer>,
        name -> Text,
        forename -> Text,
        age -> Integer,
    }
}

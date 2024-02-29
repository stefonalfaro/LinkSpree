// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    link (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
        icon -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    link,
);

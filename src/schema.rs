table! {
    errors (id) {
        id -> Integer,
        matching_string -> Varchar,
        reference_url -> Text,
        reference_case -> Text,
    }
}

table! {
    whitelist (id) {
        id -> Integer,
        matching_string -> Varchar,
        reference_url -> Text,
        reference_case -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    errors,
    whitelist,
);

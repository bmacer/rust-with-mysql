table! {
    errors (id) {
        id -> Integer,
        matching_string -> Varchar,
        reference_url -> Text,
        reference_case -> Text,
    }
}

table! {
    errtype (id) {
        id -> Integer,
        name -> Varchar,
        identifier -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    errors,
    errtype,
    posts,
);

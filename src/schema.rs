table! {
    diagnostic_files (id) {
        id -> Integer,
        diag_url -> Varchar,
        sr -> Nullable<Varchar>,
    }
}

table! {
    error_results (id) {
        id -> Integer,
        matching_string_id -> Integer,
        diagnostic_file_id -> Integer,
        count -> Integer,
    }
}

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
    diagnostic_files,
    error_results,
    errors,
    whitelist,
);

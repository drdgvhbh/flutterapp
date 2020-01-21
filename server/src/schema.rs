table! {
    deltas (id) {
        id -> Text,
        delta -> Text,
    }
}

table! {
    document_histories (id) {
        id -> Int4,
        document_id -> Text,
        delta_id -> Text,
        created_at -> Timestamp,
    }
}

table! {
    documents (id) {
        id -> Text,
        created_at -> Timestamp,
    }
}

joinable!(document_histories -> deltas (delta_id));
joinable!(document_histories -> documents (document_id));

allow_tables_to_appear_in_same_query!(
    deltas,
    document_histories,
    documents,
);

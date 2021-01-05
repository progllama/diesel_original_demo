table! {
    todos (id) {
        id -> Int4,
        task -> Text,
        complete -> Bool,
        published_at -> Nullable<Timestamp>,
    }
}

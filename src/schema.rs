table! {
    todo (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        body -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

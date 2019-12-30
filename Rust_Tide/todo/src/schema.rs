table! {
    note (id) {
        id -> Unsigned<Bigint>,
        title -> Text,
        body -> Text,
        published -> Bool,
        created -> Datetime,
        updated -> Datetime,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    note,
    posts,
);

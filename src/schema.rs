// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Integer,
        calendar -> Integer,
        year -> Nullable<Integer>,
        season -> Nullable<Integer>,
        month -> Nullable<Integer>,
        day -> Integer,
        kind -> Integer,
        fixed_date -> Bool,
        all_day -> Bool,
        title -> Nullable<Text>,
        note -> Nullable<Text>,
        datetime -> Nullable<Text>,
        duration -> Nullable<Integer>,
        status -> Nullable<Integer>,
        order_in_week -> Nullable<Text>,
        order_in_resolution -> Nullable<Text>,
        sync -> Nullable<Integer>,
        uuid -> Nullable<Text>,
    }
}

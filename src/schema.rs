table! {
    birthdays (id) {
        id -> Int4,
        date -> Date,
        person_id -> Nullable<Int4>,
    }
}

table! {
    gift_givers (gift_id, giver_id) {
        gift_id -> Int4,
        giver_id -> Int4,
    }
}

table! {
    gifts (id) {
        id -> Int4,
        name -> Varchar,
        price -> Nullable<Money>,
        birthday_id -> Nullable<Int4>,
    }
}

table! {
    parties (id) {
        id -> Int4,
        date -> Date,
        host_id -> Nullable<Int4>,
        name -> Varchar,
        birthday_id -> Nullable<Int4>,
    }
}

table! {
    party_guests (party_id, guest_id) {
        party_id -> Int4,
        guest_id -> Int4,
    }
}

table! {
    payments (id) {
        id -> Int4,
        price -> Nullable<Money>,
        vindicator_id -> Nullable<Int4>,
        vindicatee_id -> Nullable<Int4>,
        gift_id -> Nullable<Int4>,
        shopping_cart_id -> Nullable<Int4>,
    }
}

table! {
    persons (id) {
        id -> Int4,
        name -> Varchar,
        bankaccount -> Nullable<Varchar>,
    }
}

table! {
    product_consumers (product_id, consumer_id) {
        product_id -> Int4,
        consumer_id -> Int4,
    }
}

table! {
    products (id) {
        id -> Int4,
        price -> Nullable<Money>,
        quantity -> Nullable<Int4>,
        shopping_cart_id -> Nullable<Int4>,
    }
}

table! {
    shopping_carts (id) {
        id -> Int4,
        owner_id -> Nullable<Int4>,
    }
}

joinable!(birthdays -> persons (person_id));
joinable!(gift_givers -> gifts (gift_id));
joinable!(gift_givers -> persons (giver_id));
joinable!(parties -> persons (host_id));
joinable!(party_guests -> parties (party_id));
joinable!(party_guests -> persons (guest_id));
joinable!(payments -> gifts (gift_id));
joinable!(product_consumers -> persons (consumer_id));
joinable!(product_consumers -> products (product_id));
joinable!(shopping_carts -> persons (owner_id));

allow_tables_to_appear_in_same_query!(
    birthdays,
    gift_givers,
    gifts,
    parties,
    party_guests,
    payments,
    persons,
    product_consumers,
    products,
    shopping_carts,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    articles (code) {
        code -> Int8,
    }
}

diesel::table! {
    images (id) {
        id -> Int8,
        article_code -> Int8,
        image_base64_encoded -> Text,
    }
}

diesel::table! {
    prices (id) {
        id -> Int8,
        article_code -> Int8,
        price -> Numeric,
        #[max_length = 50]
        currency -> Bpchar,
    }
}

diesel::table! {
    texts (id) {
        id -> Int8,
        article_code -> Int8,
        #[max_length = 100]
        article_name -> Bpchar,
        #[max_length = 500]
        article_description -> Bpchar,
        #[max_length = 50]
        language -> Bpchar,
    }
}

diesel::joinable!(images -> articles (article_code));
diesel::joinable!(prices -> articles (article_code));
diesel::joinable!(texts -> articles (article_code));

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    images,
    prices,
    texts,
);

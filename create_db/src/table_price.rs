use std::fmt::{Debug, Formatter};

use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::prices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Prices {
    id: i64,
    article_code: i64,
    price: BigDecimal,
    currency: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::prices)]
pub struct NewPrice {
    pub(crate) article_code: i64,
    pub(crate) price: BigDecimal,
    pub(crate) currency: String,
}

impl Debug for Prices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Prices")
            .field("id", &self.id)
            .field("article_code", &self.article_code)
            .field("price", &self.price)
            .field("currency", &self.currency)
            .finish()
    }
}

impl Debug for NewPrice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewPrice")
            .field("id", &self.article_code)
            .field("currency", &self.currency)
            .field("price", &self.price)
            .finish()
    }
}

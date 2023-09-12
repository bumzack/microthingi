use std::fmt::{Debug, Formatter};

use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Image {
    id: i64,
    article_code: i64,
    image_base64_encoded: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::images)]
pub struct NewImage {
    pub article_code: i64,
    pub image_base64_encoded: String,
}

impl Debug for NewImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewImage")
            .field("code", &self.article_code)
            .finish()
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("id", &self.id)
            .field("article_code", &self.article_code)
            .finish()
    }
}


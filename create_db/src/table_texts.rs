use std::fmt::{Debug, Formatter};

use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::texts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Texts {
    id: i64,
    article_code: i64,
    article_name: String,
    article_description: String,
    language: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::texts)]
pub struct NewText {
    pub(crate) article_code: i64,
    pub(crate) article_name: String,
    pub(crate) article_description: String,
    pub(crate) language: String,
}

impl Debug for Texts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Text")
            .field("id", &self.id)
            .field("article_code", &self.article_code)
            .field("article_name", &self.article_name)
            .field("article_description", &self.article_description)
            .field("language", &self.language)
            .finish()
    }
}

impl Debug for NewText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewText")
            .field("id", &self.article_code)
            .field("article_name", &self.article_name)
            .field("article_description", &self.article_description)
            .field("language", &self.language)
            .finish()
    }
}

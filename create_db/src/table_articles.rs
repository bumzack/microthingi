use std::fmt::{Debug, Formatter};

use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Article {
    pub(crate) code: i64,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::articles)]
pub struct NewArticle {
    pub code: i64,
}

impl Debug for Article {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Article").field("code", &self.code).finish()
    }
}

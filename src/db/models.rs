use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

impl Serialize for Post {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Post", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("slug", &self.slug)?;
        state.serialize_field("tags", &self.tags)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.end()
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
// Peculiar thing about this, the order **must** match the order in the table
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub tags: Vec<Option<String>>,
    pub description: String,
    pub created_at: NaiveDateTime,
}

//! Contains abstractions related to posts; mainly db queries
use crate::db::schema::posts;
use crate::db::models::Post;
use diesel::prelude::*;
use crate::db;
//TODO: test these methods

/// Get all records in the `posts` table showing the most recent first
///
/// **Use with caution, since there's no pagination implemented yet**
///
/// TODO: Finish this documentation (I hate pagination)
pub fn all() -> Vec<Post> {
    let conn = &mut db::establish_connection();
    posts::table.order(posts::created_at.desc()).load(conn).expect("Error loading posts")
}

/// Find one `posts` in the db using its primary key
///
/// # Examples
/// ```rust
///
/// let my_id = 1;
/// if let Ok(posts) = posts::one(my_id) {
///  // do something...
/// }
/// ```
pub fn one(id: i32) -> QueryResult<Post> {
    let conn = &mut db::establish_connection();
    posts::table.find(id).first(conn)
}

/// Find one `posts` in the db using its slug
///
/// # Examples
/// ```rust
///
/// let my_id = 1;
/// if let Ok(posts) = posts::one(my_id) {
///  // do something...
/// }
/// ```
pub fn one_by_slug(slug: &str) -> QueryResult<Post> {
    let conn = &mut db::establish_connection();
    posts::table.filter(posts::slug.eq(slug)).first(conn)
}


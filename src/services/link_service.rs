use diesel::prelude::*;
use crate::models::link::{Link,  NewLink, LinkChangeset};
use crate::schema::link;

pub fn get_link_by_id(conn: &mut PgConnection, link_id: i32) -> Option<Link> {
    use crate::schema::link::dsl::*;

    link.filter(id.eq(link_id))
        .first::<Link>(conn)
        .optional() // This converts a Result into an Option.
        .expect("Error loading link by ID")
}

pub fn get_links_by_group_id(conn: &mut PgConnection, group_id_search: i32) -> Vec<Link> {
    use crate::schema::link::dsl::*;

    link.filter(group_id.eq(group_id_search))
        .load::<Link>(conn)
        .expect("Error loading links by group ID")
}

pub fn create_new_link(conn: &mut PgConnection, group_id:i32, name: Option<&str>, url: Option<&str>, color: Option<&str>, icon_param: Option<&str>) -> Link {
    let new_link = NewLink { group_id, name, url, color, icon:icon_param };

    diesel::insert_into(link::table)
        .values(&new_link)
        .get_result(conn)
        .expect("Error saving new link")
}

pub fn delete_link_by_id(conn: &mut PgConnection, link_id_to_delete: i32) -> QueryResult<usize> {
    use crate::schema::link::dsl::*;

    diesel::delete(link.filter(id.eq(link_id_to_delete)))
        .execute(conn)
}

pub fn update_link(conn: &mut PgConnection, link_id: i32, changes: LinkChangeset) -> QueryResult<Link> {
    use crate::schema::link::dsl::*;

    diesel::update(link.filter(id.eq(link_id)))
        .set(&changes)
        .get_result(conn)
}

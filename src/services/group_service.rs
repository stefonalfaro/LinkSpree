use diesel::prelude::*;
use crate::models::group::{Group, NewGroup, GroupChangeset};
use crate::schema::groups;

pub fn create_new_group(conn: &mut PgConnection, name: Option<&str>) -> Group {
    let new_group = NewGroup { name };

    diesel::insert_into(groups::table)
        .values(&new_group)
        .get_result(conn)
        .expect("Error saving new group")
}

pub fn get_all_groups(conn: &mut PgConnection) -> Vec<Group> {
    use crate::schema::groups::dsl::*;
    
    groups.load::<Group>(conn)
        .expect("Error loading groups")
}

pub fn get_group_by_id(conn: &mut PgConnection, group_id: i32) -> Option<Group> {
    use crate::schema::groups::dsl::*;
    
    groups.find(group_id)
        .first::<Group>(conn)
        .optional() // This converts a Result into an Option, making handling "not found" cases easier.
        .expect("Error loading group")
}

pub fn delete_group_by_id(conn: &mut PgConnection, group_id_to_delete: i32) -> QueryResult<usize> {
    use crate::schema::groups::dsl::*;
    
    diesel::delete(groups.filter(id.eq(group_id_to_delete)))
        .execute(conn)
}

pub fn update_group(conn: &mut PgConnection, group_id: i32, changes: GroupChangeset) -> QueryResult<Group> {
    use crate::schema::groups::dsl::*;
    
    diesel::update(groups.filter(id.eq(group_id)))
        .set(&changes)
        .get_result(conn)
}
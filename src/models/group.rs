use crate::schema::groups; // Ensure the schema module is correctly referenced
use diesel::prelude::*;

#[derive(Queryable, Debug, Clone)]
pub struct Group {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = groups)]
pub struct NewGroup<'a> {
    pub name: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = groups)]
pub struct GroupChangeset {
    pub name: Option<String>,
}
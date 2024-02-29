use crate::schema::link; // Ensure the schema module is correctly referenced
use diesel::prelude::*;

#[derive(Queryable, Debug, Clone)]
pub struct Link {
    pub id: i32,
    pub group_id: i32,
    pub name: Option<String>,
    pub url: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = link)]
pub struct NewLink<'a> {
    pub group_id: i32,
    pub name: Option<&'a str>,
    pub url: Option<&'a str>,
    pub color: Option<&'a str>,
    pub icon: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = link)]
pub struct LinkChangeset {
    pub group_id: Option<i32>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}
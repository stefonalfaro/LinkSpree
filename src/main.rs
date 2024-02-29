#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod models; // Ensure this is correct based on your project structure
mod schema; // Ensure this is correct based on your project structure

use crate::models::{Group, Link, NewGroup, NewLink}; // Adjust if necessary based on your module setup
use schema::{groups, link}; // Adjust if necessary based on your module setup

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_new_group(conn: &mut PgConnection, name: Option<&str>) -> Group {
    use schema::groups;

    let new_group = NewGroup { name };

    diesel::insert_into(groups::table)
        .values(&new_group)
        .get_result(conn)
        .expect("Error saving new group")
}

fn create_new_link(conn: &mut PgConnection, name: Option<&str>, url: Option<&str>, color: Option<&str>, icon: Option<&str>) -> Link {
    use schema::link;

    let new_link = NewLink { name, url, color, icon };

    diesel::insert_into(link::table)
        .values(&new_link)
        .get_result(conn)
        .expect("Error saving new link")
}

fn main() {
    let mut connection = establish_connection();
    // Now you can use the connection for operations

    // Example usage
    let group = create_new_group(&mut connection, Some("Example Group"));
    println!("Created new group: {:?}", group);

    let link = create_new_link(&mut connection, Some("Example Link"), Some("http://example.com"), Some("blue"), Some("icon.png"));
    println!("Created new link: {:?}", link);
}


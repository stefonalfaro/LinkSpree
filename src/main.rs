extern crate diesel;
extern crate dotenv;

mod models {
    pub mod group;
    pub mod link;
}

pub mod schema;

mod services {
    pub mod group_service;
    pub mod link_service;
}
use crate::services::group_service::{create_new_group, get_all_groups, get_group_by_id, delete_group_by_id, update_group};
use crate::services::link_service::{create_new_link, get_link_by_id, get_links_by_group_id, delete_link_by_id, update_link};

mod helpers {
    pub mod database_manager;
}
use crate::helpers::database_manager::{establish_connection};

fn main() {
    let mut connection = establish_connection();
    // Now you can use the connection for operations

    // Example usage
    let group = create_new_group(&mut connection, Some("Example Group"));
    println!("Created new group: {:?}", group);

    let link = create_new_link(&mut connection, group.id, Some("Example Link"), Some("http://example.com"), Some("blue"), Some("icon.png"));
    println!("Created new link: {:?}", link);
}
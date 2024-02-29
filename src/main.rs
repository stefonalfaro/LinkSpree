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
    pub mod config;
    pub mod authorization;
}
use crate::helpers::database_manager::{establish_connection};
use crate::helpers::config::{{ load_config, Config}};
use crate::helpers::authorization::{Authorized};

mod rest {
    pub mod group_rest;
    pub mod link_rest;
}

fn main() {
    let config = load_config().expect("Failed to load config");

    let mut connection = establish_connection();

    let group = create_new_group(&mut connection, Some("Example Group"));
    println!("Created new group: {:?}", group);

    let link = create_new_link(&mut connection, group.id, Some("Example Link"), Some("http://example.com"), Some("blue"), Some("icon.png"));
    println!("Created new link: {:?}", link);
}
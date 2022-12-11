use rocket::{Build, Rocket};
use crate::server::manifest::get;
use crate::server::a_record::{get_a, get_a_by_name, create_a, update_a, delete_a};

const A_RECORD_ROUTE: &str = "/a";
const ROOT: &str = "/";

pub(crate) fn mount() -> Rocket<Build> {
    rocket::build()
        // Manifest
        .mount(ROOT, routes![get])
        // A Records
        .mount(A_RECORD_ROUTE, routes![get_a,get_a_by_name, create_a, update_a,delete_a])
}

mod dns_manifest_parser;
mod dns_operator;
mod server;

#[macro_use] extern crate rocket;

use std::thread;
use crate::dns_operator::operator::OperatorBuilder;
use crate::dns_operator::controller::Controller;
use crate::dns_operator::runtime::Runtime;
use crate::dns_operator::watcher::Watcher;


#[launch]
fn rocket() -> _ {
    thread::spawn(|| {
        OperatorBuilder::new()
            .set_controller(Controller::new_default())
            .set_runtime(Runtime::new_coredns())
            .set_watcher(Watcher::new_file_watcher())
            .build()
            .run().expect("operator stopped unexpectedly");
    });

    server::mount::mount()
}

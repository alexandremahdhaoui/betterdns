use std::path::Path;
use notify::{Event, recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use notify::EventKind::Modify;

const DNS_MANIFEST_PATH: &str = "./dns_manifest";
const COREFILE_PATH: &str = "./corefile";



// Create a watcher to watch `FILE_DNS_PATH`
pub(crate) fn file_watcher() -> Result<(), String> {
    let (tx, rx) = channel();

    // Create a watcher to watch the current directory
    let mut watcher = recommended_watcher(tx)
        .expect("failed to create watcher");
    watch_path(&mut watcher, DNS_MANIFEST_PATH);
    watch_path(&mut watcher, COREFILE_PATH);

    await_modify_event(rx)
}

fn watch_path(watcher: &mut RecommendedWatcher, path: &str) {
    let path = Path::new(path);
    watcher
        .watch(path, RecursiveMode::NonRecursive)
        .expect("failed to watch directory");
}

// await_event takes a rx as input, loop until it receives an event.
// WARN: in the future please change this logic. This can lead to missing events in case
fn await_modify_event(rx: Receiver<Result<Event, notify::Error>>) -> Result<(), String> {
    // Loop indefinitely, waiting for notifications
    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    Ok(event) => {
                        if let Modify(_) = event.kind { return Ok(()) }
                    },
                    Err(e) => println!("error: {:?}", e)
                }
            }
            Err(e) => println!("error: {:?}", e)
        }
    }
}
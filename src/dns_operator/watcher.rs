use crate::dns_operator::watcher_file::file_watcher;

pub(crate) enum Event {
    Create,
    Update,
    Delete
}

// Bridge for different DNS server runtime.
pub(crate) enum Watcher {
    Null,
    File(FileWatcher),
}

impl Watcher {
    pub(crate) fn new() -> Self { Self::Null }
    pub(crate) fn new_file_watcher() -> Self {Self::File(FileWatcher::new())}

    pub(crate) fn watch(&self) -> Result<Event, String> {
        match self {
            Watcher::Null => unimplemented!(),
            Watcher::File(watcher) => watcher.watch()
        }
    }

    pub(crate) fn unwatch(&self) -> Result<(), String> {
        match self {
            Watcher::Null => unimplemented!(),
            Watcher::File(watcher) => watcher.unwatch()
        }
    }
}

// FileWatcher
pub(crate) struct FileWatcher {}

impl FileWatcher {
    fn new() -> Self { Self{} }

    fn watch(&self) -> Result<Event, String> {
        file_watcher()?;
        Ok(Event::Update)
    }
    fn unwatch(&self) -> Result<(), String> { unimplemented!()}
}
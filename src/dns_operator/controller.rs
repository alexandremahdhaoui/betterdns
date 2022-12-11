use crate::dns_operator::runtime::Runtime;
use crate::dns_operator::watcher::{Event, Watcher};

// ------------------------------------------ Controller -------------------------------------------

// Bridge for different DNS server runtime.
pub(crate) enum Controller {
    Null,
    Default(DefaultController),
}

impl Controller {
    pub(crate) fn new() -> Self { Self::Null }
    pub(crate) fn new_default() -> Self { Self::Default(DefaultController::new()) }

    pub(crate) fn reconcile(
        &self,
        event: Event,
        runtime: &mut Runtime,
        watcher: &mut Watcher
    ) -> Result<(), String> {
        match self {
            Controller::Null => unimplemented!(),
            Controller::Default(controller) => {
                controller.reconcile(event, runtime, watcher)
            }
        }
    }
}
// ---------------------------------------- DefaultRuntime -----------------------------------------

pub(crate) struct DefaultController {}

impl DefaultController {
    pub(crate) fn new() -> Self {Self{}}

    fn reconcile(
        &self,
        event: Event,
        runtime: &mut Runtime,
        _watcher: &mut Watcher
    ) -> Result<(), String> {
        match event {
            Event::Create => unimplemented!(),
            Event::Update => {
                runtime.shutdown()?;
                runtime.run()
            },
            Event::Delete => unimplemented!()
        }
    }
}
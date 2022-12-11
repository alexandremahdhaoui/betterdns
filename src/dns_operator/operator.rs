use crate::dns_operator::controller::Controller;
use crate::dns_operator::runtime::Runtime;
use crate::dns_operator::watcher::Watcher;

// ------------------------------------------- Builder ---------------------------------------------

pub(crate) struct OperatorBuilder {
    operator: Operator
}

impl OperatorBuilder {
    pub(crate) fn new() -> Self { Self{ operator: Operator::new() } }

    pub(crate) fn build(&mut self) -> &mut Operator { &mut self.operator }

    pub(crate) fn set_controller(&mut self, controller: Controller) -> &mut Self {
        self.operator.controller = controller;
        self
    }

    pub(crate) fn set_runtime(&mut self, runtime: Runtime) -> &mut Self {
        self.operator.runtime = runtime;
        self
    }
    pub(crate) fn set_watcher(&mut self, watcher: Watcher) -> &mut Self {
        self.operator.watcher = watcher;
        self
    }
}

// ------------------------------------------ Controller -------------------------------------------

pub(crate) struct Operator {
    // controller is a strategy for different controller.
    controller: Controller,
    // runtime is bridge for different DNS server runtime.
    runtime: Runtime,
    // watcher is a strategy for different observer implementation.
    watcher: Watcher,
}

impl Operator {
    fn new() -> Self {
       Self{
           runtime: Runtime::new(),
           watcher: Watcher::new(),
           controller: Controller::new(),
       }
    }

    pub(crate) fn run(&mut self) -> Result<(), String> {
        self.runtime.run()?;
        loop {
            match self.watcher.watch() {
                Ok(event) => {
                    self.controller.reconcile(event, &mut self.runtime, &mut self.watcher)?;
                }
                Err(e) => return Err(e)
            }
        }
    }

    pub(crate) fn shutdown(&mut self) -> Result<(), String> {
        self.runtime.shutdown()
    }
}


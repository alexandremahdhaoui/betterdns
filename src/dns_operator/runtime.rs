use std::process::{Child, Command};

const COREDNS_BIN: &str = "coredns";

// Bridge for different DNS server runtime.
pub(crate) enum Runtime {
    Null,
    CoreDNS(CoreDNSRuntime)
}

impl Runtime {
    pub(crate) fn new() -> Self { Self::Null }
    pub(crate) fn new_coredns() -> Self { Self::CoreDNS(CoreDNSRuntime::new())}

    pub(crate) fn run(&mut self) -> Result<(), String> {
        match self {
            Runtime::Null => unimplemented!(),
            Runtime::CoreDNS(runtime) => runtime.run()
        }
    }

    pub(crate) fn shutdown(&mut self) -> Result<(), String> {
        match self {
            Runtime::Null => unimplemented!(),
            Runtime::CoreDNS(ref mut runtime) => return runtime.shutdown()
        }
    }
}

pub(crate) struct CoreDNSRuntime {
    process: Option<Child>
}

impl CoreDNSRuntime {
    pub(crate) fn new() -> Self { Self{ process: None } }

    fn run(&mut self) -> Result<(), String> {
        // Start a process (the "sleep" command in this case, which just waits for a specified amount of time)
        let process = Command::new(COREDNS_BIN)
            // Start the process and capture the `Child` struct
            .spawn()
            .expect("failed to start process");
        self.process = Some(process);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        match self.process {
            None => Err("process should be running to shutdown".to_string()),
            Some(ref mut process) => {
                if process.kill().is_err() {
                    panic!("error while shutting down CoreDNSRuntime")
                }
                Ok(())
            }
        }
    }
}





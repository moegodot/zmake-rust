use std::cell::Cell;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::LazyLock;
use tracing::{trace, trace_span};

#[derive(Debug)]
pub struct EngineOptions {
    pub tokio_runtime: tokio::runtime::Runtime,
}

#[derive(Debug)]
pub struct Engine {
    pub runtime: v8::OwnedIsolate,
    pub tokio_runtime: tokio::runtime::Runtime,
}

impl Engine {
    pub fn new_resolve_engine(options: EngineOptions) -> eyre::Result<Self> {
        todo!()
    }

    pub fn execute_and_to_json(&mut self, code: &str, source_name: &str) -> std::string::String {
        "".to_string()
    }
}

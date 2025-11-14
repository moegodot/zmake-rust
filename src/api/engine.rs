use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::LazyLock;
use rustyscript::deno_core::v8;
use rustyscript::deno_core::v8::{OwnedIsolate, Platform, SharedRef};
use rustyscript::Runtime;
use tracing_subscriber::util::SubscriberInitExt;

pub static PLATFORM: LazyLock<SharedRef<Platform>> = LazyLock::new(|| {
    let platform =
        v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform.clone());
    v8::V8::initialize();
    platform
});

pub struct Engine{
    pub runtime:Box<Runtime>
}

impl Default for Engine{
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self{
        Engine{
            runtime: Default::default()
        }
    }

    pub fn execute(&mut self, code:String, source_name:String){

    }
}

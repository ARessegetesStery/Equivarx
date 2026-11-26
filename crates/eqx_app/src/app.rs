use crate::module::Module;
use crate::runtime::IoRuntime;
use log::{info, LevelFilter};

pub struct App {
    pub(crate) enabled_modules: Vec<Box<dyn Module>>,
    runtime: IoRuntime,
}

// TODO: add macros to enable loading default modules/custom modules
impl App {
    pub fn init() -> Self {
        env_logger::builder().filter(None, LevelFilter::Info).init();
        let runtime = IoRuntime::init();
        info!("App initialized");
        Self {
            enabled_modules: Vec::new(),
            runtime,
        }
    }

    pub fn runtime(&self) -> &IoRuntime {
        &self.runtime
    }

    pub fn load_modules(&mut self, mut modules: Vec<Box<dyn Module>>) -> &mut Self {
        modules.iter_mut().for_each(|module| module.setup());
        self.enabled_modules.extend(modules);
        self
    }

    pub fn run(&mut self) {
        for module in &mut self.enabled_modules {
            module.init(&self.runtime);
        }
    }
}

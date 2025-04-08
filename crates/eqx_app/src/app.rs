use crate::module::Module;
use env_logger;
use log::{info, LevelFilter};

pub struct App {
    enabled_modules: Vec<Box<dyn Module>>,
}

impl App {
    pub fn init() -> Self {
        env_logger::builder().filter(None, LevelFilter::Info).init();
        info!("App initialized");
        Self::default()
    }

    fn add_module(&mut self, module: Box<dyn Module>) {
        self.enabled_modules.push(module);
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            enabled_modules: vec![],
        }
    }
}

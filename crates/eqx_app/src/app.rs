use crate::module::Module;
use log::{info, LevelFilter};

#[derive(Default)]
pub struct App {
    pub(crate) enabled_modules: Vec<Box<dyn Module>>,
}

// TODO: add macros to enable loading default modules/custom modules
impl App {
    pub fn init() -> Self {
        env_logger::builder().filter(None, LevelFilter::Info).init();
        info!("App initialized");
        Self::default()
    }

    pub fn load_modules(&mut self, mut modules: Vec<Box<dyn Module>>) -> &mut Self {
        modules.iter_mut().for_each(|module| module.setup());
        self.enabled_modules.extend(modules);
        self
    }

    pub fn run(&mut self) {
        for module in &mut self.enabled_modules {
            module.setup();
            module.init();
        }
    }
}

use crate::runtime::IoRuntime;

pub trait Module {
    /// Actions the module need to do itself, before it sees any other module
    fn setup(&mut self);

    /// Initializations modules need. This could have dependencies on other modules
    fn init(&mut self, runtime: &IoRuntime);

    fn update(&mut self, runtime: &IoRuntime);
}

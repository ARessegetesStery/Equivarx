pub trait Module {
    /// Called by [`App`] upon being added to the [`enabled_modules`] list.
    fn setup(&mut self);

    /// Called by [`App`] upon its launch.
    fn init(&mut self);
}

pub trait Module {
    fn setup(&mut self);

    fn init(&mut self);
}

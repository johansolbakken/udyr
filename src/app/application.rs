pub struct Application {
    running: bool,
}

impl Application {
    pub fn new() -> Application {
        return Application { running: true };
    }

    pub fn run(&mut self) {
        while self.running {}
    }

    pub fn destroy(&mut self) {}
}

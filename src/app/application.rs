
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_init() -> Result<(), String> {
        let app = Application::new();
        assert_eq!(app.running, true);
        Ok(())
    }
}
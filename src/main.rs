mod app;

fn main() {
    let mut app = app::application::Application::new();
    app.run();
    app.destroy();
}

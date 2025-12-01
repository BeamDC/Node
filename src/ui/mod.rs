slint::include_modules!();

pub struct UI {
    window: MainWindow,
}

impl UI {
    /// create a new [`UI`]
    pub fn new() -> Self {
        let window = MainWindow::new().unwrap();
        Self {
            window
        }
    }

    /// run the [`UI`]
    pub fn run(&mut self) {
        self.window.run().unwrap()
    }
}
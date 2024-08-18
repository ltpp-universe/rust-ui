slint::include_modules!();

pub fn run() {
    let main_window: MainWindow = MainWindow::new().unwrap();
    main_window.run().unwrap();
}

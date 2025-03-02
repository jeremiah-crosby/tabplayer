mod ui;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use ui::MainWindow;

const APP_ID: &str = "com.jeremiahcrosby.TabPlayer";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("tabplayer.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = MainWindow::new(app);
    window.present();
}
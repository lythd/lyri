use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Label};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let label = Label::with_mnemonic("hello");
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Lyri")
        .child(&label)
        .build();

    // Present window
    window.present();
}
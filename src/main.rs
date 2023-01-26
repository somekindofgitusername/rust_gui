use std::env;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
const APP_ID: &str = "org.gtk_rs.HelloWorld3";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .opacity(0.9)
        .build();

    // Show the button
    button.show();


    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to &button_text()
        button.set_label(&button_text());
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(350)
        .default_height(70)
        .child(&button)
        .build();

    // Present window
    window.present();
}


fn button_text()  -> String{
    //"Hello, Worldyyy!"      
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.to_str().unwrap();
    current_dir_str.to_string()
}

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
 
    // Create a window and add a button to it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello, World!")
        .build();

    //let button = Button::with_label(&button_text());
    // Create a button that says "Open a directory"
    let button = Button::with_label("Open a directory");
    // show the button
    button.show();
    // Add an area below the button and display text
    let label = gtk::Label::new(Some("label"));
    label.show();
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&button, true, true, 0);
    vbox.pack_start(&label, true, true, 0);
    vbox.show();
    window.set_child(Some(&vbox));
    

    window.set_child(Some(&button));
    // When the button is clicked a File Dialog that selects a directory will be opened
    button.connect_clicked(move |_| {
        let label = label.clone();
        // dialog is a gtk::FileChooserDialog that selects a directory
        let dialog = gtk::FileChooserDialog::new(
            Some("Open Directory"),
            None::<&gtk::Window>,
            gtk::FileChooserAction::SelectFolder,
        );


        dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());
        dialog.add_button("Open", gtk::ResponseType::Ok.into());
        dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Ok.into() {
                if let Some(file) = dialog.file() {
                    // set the label to the path of the file
                    label.set_text(&file.path().unwrap().to_str().unwrap());
                }
            }
            dialog.close();
        });
        dialog.show();
    });
    

    // Show the window and all of its contents
    window.show();

    
}


fn button_text()  -> String{   
    let current_dir = env::current_dir().unwrap();
    let current_dir_str = current_dir.to_str().unwrap();
    current_dir_str.to_string()
}

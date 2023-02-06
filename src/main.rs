use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
const APP_ID: &str = "filepatterwatcher";
mod filepatterns;
use filepatterns::*;

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
    // Set minimum size of the window
    window.set_default_size(350, 70);

    // Create a button that says "Open a directory"
    let button = Button::with_label("Open a directory");
    button.show();

    // Add an area below the button and display text
    let label = gtk::Label::new(Some("label"));
    label.show();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&button, true, true, 0);
    vbox.pack_start(&label, true, true, 0);
    vbox.show();

    window.set_child(Some(&vbox));

    // When the button is clicked a File Dialog that selects a directory will be opened
    button.connect_clicked(move |_| {
        let label = label.clone();
        // dialog is a gtk::FileChooserDialog that selects a directory
        let dialog = gtk::FileChooserDialog::new(
            Some("Open Directory"),
            None::<&gtk::Window>,
            gtk::FileChooserAction::SelectFolder,
        );
        dialog.add_button("Cancel", gtk::ResponseType::Cancel);
        dialog.add_button("Open", gtk::ResponseType::Ok);
        dialog.connect_response(move |dialog, response| {
                if response == gtk::ResponseType::Ok {
                    //let mut files: Vec<String> = vec![];
                if let Some(file) = dialog.file() {               
                    // a vector of strings with the file names
                    let file_path = file.path().unwrap();
                    let dir_path = file_path.to_str().unwrap();
                    let files = read_dir(dir_path);
                    // a vector of strings with the filtered file names
                    let filtered_files = filter_strings(files);
                    // groups of files 
                    let groups = group_strings(filtered_files);
                    // bind a variable to the filtered files if their base is not empty
                    let groups = groups.into_iter().filter(|(base, _)| !base.is_empty());
                    //flatten the groups but keep them visually separated and the base name visually on top
                    let groups = groups.flat_map(|(base, files)| {
                        let mut files = files;
                        files.insert(0, base);
                        files
                    });
                    //set the label to the flattened groups
                    label.set_text(&groups.collect::<Vec<_>>().join("\n"));
                }
            }
            dialog.close();
        });
        dialog.show();
    });
    // Show the window and all of its contents
    window.show();
}

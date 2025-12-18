use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, FileChooserAction, FileChooserDialog,
    FontChooserDialog, MenuButton, Orientation, ScrolledWindow, TextView,
};
use gio::prelude::*;
use gio::Menu;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use glib::clone;

fn main() {
    let app = Application::builder()
        .application_id("com.izhan.textwriter")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let text_view = TextView::new();
    text_view.set_monospace(true);

    let buffer = text_view.buffer().unwrap();
    let current_file = Rc::new(RefCell::new(None::<String>));

    let scroll = ScrolledWindow::builder()
        .child(&text_view)
        .build();

    let file_menu = Menu::new();
    file_menu.append(Some("New"), Some("app.new"));
    file_menu.append(Some("New Window"), Some("app.new_window"));
    file_menu.append(Some("Open"), Some("app.open"));
    file_menu.append(Some("Save"), Some("app.save"));
    file_menu.append(Some("Save As"), Some("app.save_as"));
    file_menu.append(Some("Exit"), Some("app.exit"));

    let font_menu = Menu::new();
    font_menu.append(Some("Change Font"), Some("app.change_font"));

    let help_menu = Menu::new();
    help_menu.append(Some("About TextWriter"), Some("app.about"));

    let file_button = MenuButton::builder()
        .label("File")
        .menu_model(&file_menu)
        .build();

    let font_button = MenuButton::builder()
        .label("Font")
        .menu_model(&font_menu)
        .build();

    let help_button = MenuButton::builder()
        .label("Help")
        .menu_model(&help_menu)
        .build();

    let hbox = GtkBox::new(Orientation::Horizontal, 5);
    hbox.append(&file_button);
    hbox.append(&font_button);
    hbox.append(&help_button);

    let vbox = GtkBox::new(Orientation::Vertical, 0);
    vbox.append(&hbox);
    vbox.append(&scroll);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("TextWriter")
        .default_width(900)
        .default_height(600)
        .child(&vbox)
        .build();

    let buffer_clone = buffer.clone();
    let text_view_clone = text_view.clone();
    let current_file_clone = current_file.clone();
    let window_clone = window.clone();

    let action_group = gio::SimpleActionGroup::new();
    window.insert_action_group("app", Some(&action_group));

    let new_action = gio::SimpleAction::new("new", None);
    new_action.connect_activate(clone!(@strong buffer => move |_, _| {
        buffer.set_text("");
    }));
    action_group.add_action(&new_action);

    let new_window_action = gio::SimpleAction::new("new_window", None);
    new_window_action.connect_activate(clone!(@strong app => move |_, _| {
        build_ui(&app);
    }));
    action_group.add_action(&new_window_action);

    let open_action = gio::SimpleAction::new("open", None);
    open_action.connect_activate(clone!(@strong buffer_clone, @strong current_file_clone, @strong window_clone => move |_, _| {
        let dialog = FileChooserDialog::builder()
            .transient_for(&window_clone)
            .title("Open File")
            .action(FileChooserAction::Open)
            .build();
        dialog.add_buttons(&[("Open", gtk4::ResponseType::Accept), ("Cancel", gtk4::ResponseType::Cancel)]);
        dialog.connect_response(clone!(@strong buffer_clone, @strong current_file_clone => move |d, resp| {
            if resp == gtk4::ResponseType::Accept {
                if let Some(path) = d.file().and_then(|f| f.path()) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        buffer_clone.set_text(&content);
                        *current_file_clone.borrow_mut() = Some(path.to_string_lossy().to_string());
                    }
                }
            }
            d.close();
        }));
        dialog.present();
    }));
    action_group.add_action(&open_action);

    let save_action = gio::SimpleAction::new("save", None);
    save_action.connect_activate(clone!(@strong buffer_clone, @strong current_file_clone => move |_, _| {
        if let Some(path) = current_file_clone.borrow().clone() {
            let text = buffer_clone.text(&buffer_clone.start_iter(), &buffer_clone.end_iter(), false);
            let _ = fs::write(path, text);
        }
    }));
    action_group.add_action(&save_action);

    let save_as_action = gio::SimpleAction::new("save_as", None);
    save_as_action.connect_activate(clone!(@strong buffer_clone, @strong current_file_clone, @strong window_clone => move |_, _| {
        let dialog = FileChooserDialog::builder()
            .transient_for(&window_clone)
            .title("Save File")
            .action(FileChooserAction::Save)
            .build();
        dialog.add_buttons(&[("Save", gtk4::ResponseType::Accept), ("Cancel", gtk4::ResponseType::Cancel)]);
        dialog.connect_response(clone!(@strong buffer_clone, @strong current_file_clone => move |d, resp| {
            if resp == gtk4::ResponseType::Accept {
                if let Some(path) = d.file().and_then(|f| f.path()) {
                    let text = buffer_clone.text(&buffer_clone.start_iter(), &buffer_clone.end_iter(), false);
                    let _ = fs::write(&path, text);
                    *current_file_clone.borrow_mut() = Some(path.to_string_lossy().to_string());
                }
            }
            d.close();
        }));
        dialog.present();
    }));
    action_group.add_action(&save_as_action);

    let exit_action = gio::SimpleAction::new("exit", None);
    exit_action.connect_activate(clone!(@strong window_clone => move |_, _| {
        window_clone.close();
    }));
    action_group.add_action(&exit_action);

    let change_font_action = gio::SimpleAction::new("change_font", None);
    change_font_action.connect_activate(clone!(@strong buffer_clone, @strong window_clone => move |_, _| {
        let dialog = FontChooserDialog::builder()
            .transient_for(&window_clone)
            .title("Choose Font")
            .build();
        dialog.connect_response(clone!(@strong buffer_clone => move |d, resp| {
            if resp == gtk4::ResponseType::Ok {
                if let Some(font_name) = d.font() {
                    let tag = gtk4::TextTag::new(Some("font_tag"));
                    tag.set_property("font", &font_name);
                    buffer_clone.tag_table().add(&tag);
                    buffer_clone.apply_tag(&tag, &buffer_clone.start_iter(), &buffer_clone.end_iter());
                }
            }
            d.close();
        }));
        dialog.present();
    }));
    action_group.add_action(&change_font_action);

    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(clone!(@strong window_clone => move |_, _| {
        let dialog = gtk4::AboutDialog::builder()
            .program_name("TextWriter")
            .version("0.1.0")
            .comments("Lightweight LXDE text editor written in Rust!")
            .authors(&["Izhan"])
            .license_type(gtk4::License::MitX11)
            .transient_for(&window_clone)
            .build();
        dialog.present();
    }));
    action_group.add_action(&about_action);

    window.present();
}

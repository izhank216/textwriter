use gtk4::prelude::*;
use gtk4::{
    AboutDialog, Application, ApplicationWindow, Box as GtkBox,
    FileChooserAction, FileChooserDialog, MenuBar, MenuItem,
    Orientation, ResponseType, ScrolledWindow, TextView,
    FontChooserDialog,
};
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

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

    let menubar = MenuBar::new();

    let file_menu = gtk4::Menu::new();
    let file_item = MenuItem::with_label("File");
    file_item.set_submenu(Some(&file_menu));

    let new_item = MenuItem::with_label("New");
    new_item.connect_activate({
        let buffer = buffer.clone();
        move |_| buffer.set_text("")
    });

    let new_window_item = MenuItem::with_label("New Window");
    new_window_item.connect_activate({
        let app = app.clone();
        move |_| build_ui(&app)
    });

    let open_item = MenuItem::with_label("Open");
    open_item.connect_activate({
        let buffer = buffer.clone();
        let current_file = current_file.clone();
        move |_| {
            let dialog = FileChooserDialog::new(
                Some("Open File"),
                None::<&ApplicationWindow>,
                FileChooserAction::Open,
                &[("Open", ResponseType::Accept), ("Cancel", ResponseType::Cancel)],
            );
            if dialog.run() == ResponseType::Accept {
                if let Some(path) = dialog.file().and_then(|f| f.path()) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        buffer.set_text(&content);
                        *current_file.borrow_mut() = Some(path.to_string_lossy().to_string());
                    }
                }
            }
            dialog.close();
        }
    });

    let save_item = MenuItem::with_label("Save");
    save_item.connect_activate({
        let buffer = buffer.clone();
        let current_file = current_file.clone();
        move |_| {
            if let Some(path) = current_file.borrow().clone() {
                let text = buffer.text(
                    &buffer.start_iter(),
                    &buffer.end_iter(),
                    false,
                );
                let _ = fs::write(path, text);
            }
        }
    });

    let save_as_item = MenuItem::with_label("Save As");
    save_as_item.connect_activate({
        let buffer = buffer.clone();
        let current_file = current_file.clone();
        move |_| {
            let dialog = FileChooserDialog::new(
                Some("Save File"),
                None::<&ApplicationWindow>,
                FileChooserAction::Save,
                &[("Save", ResponseType::Accept), ("Cancel", ResponseType::Cancel)],
            );
            if dialog.run() == ResponseType::Accept {
                if let Some(path) = dialog.file().and_then(|f| f.path()) {
                    let text = buffer.text(
                        &buffer.start_iter(),
                        &buffer.end_iter(),
                        false,
                    );
                    let _ = fs::write(&path, text);
                    *current_file.borrow_mut() = Some(path.to_string_lossy().to_string());
                }
            }
            dialog.close();
        }
    });

    let exit_item = MenuItem::with_label("Exit");
    exit_item.connect_activate(|_| gtk4::main_quit());

    file_menu.append(&new_item);
    file_menu.append(&new_window_item);
    file_menu.append(&open_item);
    file_menu.append(&save_item);
    file_menu.append(&save_as_item);
    file_menu.append(&exit_item);

    let font_menu = gtk4::Menu::new();
    let font_item = MenuItem::with_label("Font");
    font_item.set_submenu(Some(&font_menu));

    let change_font_item = MenuItem::with_label("Change Font");
    change_font_item.connect_activate({
        let text_view = text_view.clone();
        move |_| {
            let dialog = FontChooserDialog::new(Some("Choose Font"), None::<&ApplicationWindow>);
            if dialog.run() == ResponseType::Ok {
                if let Some(font) = dialog.font() {
                    text_view.set_font_map(None);
                    text_view.style_context().add_class(&font);
                }
            }
            dialog.close();
        }
    });

    let change_font_size_item = MenuItem::with_label("Change Font Size");
    change_font_size_item.connect_activate({
        let text_view = text_view.clone();
        move |_| {
            let dialog = FontChooserDialog::new(Some("Change Font Size"), None::<&ApplicationWindow>);
            if dialog.run() == ResponseType::Ok {
                if let Some(font) = dialog.font() {
                    text_view.style_context().add_class(&font);
                }
            }
            dialog.close();
        }
    });

    font_menu.append(&change_font_item);
    font_menu.append(&change_font_size_item);

    let help_menu = gtk4::Menu::new();
    let help_item = MenuItem::with_label("Help");
    help_item.set_submenu(Some(&help_menu));

    let about_item = MenuItem::with_label("About TextWriter");
    about_item.connect_activate(move |_| {
        let dialog = AboutDialog::builder()
            .program_name("TextWriter")
            .version("0.1.0")
            .comments("Lightweight LXDE text editor written in Rust!")
            .authors(vec!["Izhan"])
            .license_type(gtk4::License::MitX11)
            .build();
        dialog.show();
    });

    help_menu.append(&about_item);

    menubar.append(&file_item);
    menubar.append(&font_item);
    menubar.append(&help_item);

    let vbox = GtkBox::new(Orientation::Vertical, 0);
    vbox.append(&menubar);
    vbox.append(&scroll);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("TextWriter")
        .default_width(900)
        .default_height(600)
        .child(&vbox)
        .build();

    window.show();
}

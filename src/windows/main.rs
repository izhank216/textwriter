#![cfg(target_os = "windows")]
#![windows_subsystem = "windows"]

use native_windows_derive::NwgUi;
use native_windows_gui as nwg;
use nwg::NativeUi;
use std::cell::RefCell;
use std::fs;

#[derive(Default, NwgUi)]
pub struct TextWriterApp {
    #[nwg_control(title: "TextWriter", size: (900, 600), position: (300, 200))]
    #[nwg_events( OnWindowClose: [TextWriterApp::exit] )]
    window: nwg::Window,

    #[nwg_control(parent: window, text: "File")]
    file_menu: nwg::Menu,

    #[nwg_control(parent: file_menu, text: "New")]
    #[nwg_events( OnMenuItemSelected: [TextWriterApp::new_file] )]
    menu_new: nwg::MenuItem,

    #[nwg_control(parent: file_menu, text: "Open")]
    #[nwg_events( OnMenuItemSelected: [TextWriterApp::open_file] )]
    menu_open: nwg::MenuItem,

    #[nwg_control(parent: file_menu, text: "Save")]
    #[nwg_events( OnMenuItemSelected: [TextWriterApp::save_file] )]
    menu_save: nwg::MenuItem,

    #[nwg_control(parent: file_menu, text: "Save As")]
    #[nwg_events( OnMenuItemSelected: [TextWriterApp::save_file_as] )]
    menu_save_as: nwg::MenuItem,

    #[nwg_control(parent: file_menu, text: "Exit")]
    #[nwg_events( OnMenuItemSelected: [TextWriterApp::exit] )]
    menu_exit: nwg::MenuItem,

    #[nwg_control(parent: window, text: "Help")]
    help_menu: nwg::Menu,

    #[nwg_control(parent: help_menu, text: "About TextWriter")]
    #[nwg_events( OnMenuItemSelected: [TextWriterApp::about] )]
    menu_about: nwg::MenuItem,

    #[nwg_control(parent: window, flags: "VISIBLE | VSCROLL | HSCROLL | AUTOVSCROLL")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0)]
    text_edit: nwg::TextBox,

    #[nwg_layout(parent: window, max_row: 1, max_col: 1)]
    grid: nwg::GridLayout,

    current_path: RefCell<Option<String>>,
}

impl TextWriterApp {
    fn new_file(&self) {
        self.text_edit.set_text("");
        *self.current_path.borrow_mut() = None;
    }

    fn open_file(&self) {
        let mut dialog = nwg::FileDialog::default();
        if nwg::FileDialog::builder()
            .title("Open File")
            .action(nwg::FileDialogAction::Open)
            .build(&mut dialog)
            .is_ok() 
            && dialog.run(Some(&self.window)) 
        {
            if let Ok(path) = dialog.get_selected_item() {
                if let Ok(content) = fs::read_to_string(&path) {
                    self.text_edit.set_text(&content);
                    *self.current_path.borrow_mut() = Some(path);
                }
            }
        }
    }

    fn save_file(&self) {
        let path_opt = self.current_path.borrow().clone();
        if let Some(path) = path_opt {
            let text = self.text_edit.text();
            let _ = fs::write(path, text);
        } else {
            self.save_file_as();
        }
    }

    fn save_file_as(&self) {
        let mut dialog = nwg::FileDialog::default();
        if nwg::FileDialog::builder()
            .title("Save File")
            .action(nwg::FileDialogAction::Save)
            .build(&mut dialog)
            .is_ok() 
            && dialog.run(Some(&self.window)) 
        {
            if let Ok(path) = dialog.get_selected_item() {
                let text = self.text_edit.text();
                let _ = fs::write(&path, text);
                *self.current_path.borrow_mut() = Some(path);
            }
        }
    }

    fn about(&self) {
        nwg::modal_info_message(
            &self.window,
            "About TextWriter",
            "TextWriter is a lightweight LXDE (Windows Version) text editor written in Rust!",
        );
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .size(16)
        .family("Consolas")
        .build(&mut font)
        .ok();
    nwg::Font::set_global_default(Some(font));

    let _app = TextWriterApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

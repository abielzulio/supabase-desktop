#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{api::{shell}};
use tauri::{Manager, CustomMenuItem, Menu, MenuItem, Submenu, AboutMetadata, MenuEntry};

pub fn custom_window_menu() -> Menu {
  let app_name: &str = "Supabase";
  let menu = Menu::with_items([

    #[cfg(target_os = "macos")]
    MenuEntry::Submenu(Submenu::new(
      app_name,
      Menu::with_items([
        MenuItem::About(app_name.to_string(), AboutMetadata::default()).into(),
        MenuItem::Separator.into(),
        MenuItem::Services.into(),
        MenuItem::Separator.into(),
        MenuItem::Hide.into(),
        MenuItem::HideOthers.into(),
        MenuItem::ShowAll.into(),
        MenuItem::Separator.into(),
        MenuItem::Quit.into(),
      ]))),

    #[cfg(not(target_os = "macos"))] {
      MenuEntry::Submenu(Submenu::new(
        "File",
        Menu::with_items([
          MenuItem::Quit.into()
        ])
      ));
    },

    MenuEntry::Submenu(Submenu::new(
      "Window",
      Menu::with_items([
        MenuItem::Minimize.into(),
        #[cfg(target_os = "macos")]
          MenuItem::Zoom.into(),
          MenuItem::Separator.into(),
          MenuItem::EnterFullScreen.into(),
          MenuItem::Separator.into(),
        MenuItem::CloseWindow.into(),
      ]),
    )),

    MenuEntry::Submenu(Submenu::new(
      "About",
      Menu::with_items([
        CustomMenuItem::new("github".to_string(), "View Source Code").into(),
        CustomMenuItem::new("twitter".to_string(), "Author on Twitter").into()
      ])
    )),
      
  ]);

  menu
}

fn main() {
  tauri::Builder::default()
    .menu(custom_window_menu())
    .on_menu_event(|event| {
      match event.menu_item_id() {
        "github" => {
          shell::open(
            &event.window().get_window("main").unwrap().shell_scope(),
            "https://github.com/abielzulio/supabase-desktop".to_string(),
            None,
          ).unwrap()
        }
        "twitter" => {
          shell::open(
            &event.window().get_window("main").unwrap().shell_scope(),
            "https://twitter.com/abielzulio".to_string(),
            None,
          ).unwrap()
        }
        _ => {}
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
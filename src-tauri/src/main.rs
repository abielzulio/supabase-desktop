#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use tauri::{api, CustomMenuItem, Menu, MenuItem, Submenu, AboutMetadata};
use tauri::{Manager};

pub fn custom_window_menu() -> Menu {
  let app_name: &str = "Supabase";
  let mut menu = Menu::new();

  #[cfg(target_os = "macos")]
  {
    menu = menu.add_submenu(Submenu::new(
      app_name,
      Menu::new()
        .add_native_item(MenuItem::About(
          app_name.to_string(),
          AboutMetadata::default(),
        ))
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Services)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit),
    ));
  }

  #[cfg(not(target_os = "macos"))]
  {
    menu = menu.add_submenu(Submenu::new(
      "File",
      Menu::new().add_native_item(MenuItem::Quit),
    ));
  }

  let mut window_menu = Menu::new();
  window_menu = window_menu.add_native_item(MenuItem::Minimize);
  #[cfg(target_os = "macos")]
  {
    window_menu = window_menu.add_native_item(MenuItem::Zoom);
    window_menu = window_menu.add_native_item(MenuItem::Separator);
    window_menu = window_menu.add_native_item(MenuItem::EnterFullScreen);
    window_menu = window_menu.add_native_item(MenuItem::Separator);
  }
  window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
  menu = menu.add_submenu(Submenu::new("Window", window_menu));

  let help_menu = Menu::new();
  let github = CustomMenuItem::new("github".to_string(), "View Source Code");
  let twitter = CustomMenuItem::new("twitter".to_string(), "Author on Twitter");
  menu = menu.add_submenu(Submenu::new("About", help_menu.add_item(github).add_item(twitter)));

  menu
}

fn main() {
  tauri::Builder::default()
    .menu(custom_window_menu())
    .on_menu_event(|event| {
      match event.menu_item_id() {
        "github" => {
          api::shell::open(
            &event.window().get_window("main").unwrap().shell_scope(),
            "https://github.com/abielzulio/supabase-desktop".to_string(),
            None,
          ).unwrap()
        }
        "twitter" => {
          api::shell::open(
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
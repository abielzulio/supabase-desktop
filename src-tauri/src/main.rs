#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use tauri::{api::{shell}, Context, utils::assets::EmbeddedAssets};
use tauri::{Manager, CustomMenuItem, Menu, MenuItem, Submenu, MenuEntry};

#[cfg(target_os = "macos")]
use tauri::{AboutMetadata};


pub fn custom_window_menu(ctx: &Context<EmbeddedAssets>) -> Menu {
  let menu = Menu::with_items([

    #[cfg(target_os = "macos")]
    MenuEntry::Submenu(Submenu::new(
      ctx.package_info().name.to_string(),
      Menu::with_items([
        MenuItem::About(ctx.package_info().name.to_string(), AboutMetadata::default()).into(),
        MenuItem::Separator.into(),
        MenuItem::Services.into(),
        MenuItem::Separator.into(),
        MenuItem::Hide.into(),
        MenuItem::HideOthers.into(),
        MenuItem::ShowAll.into(),
        MenuItem::Separator.into(),
        MenuItem::Quit.into(),
      ]))),

    MenuEntry::Submenu(Submenu::new(
      "Edit",
      Menu::with_items([
        MenuItem::Undo.into(),
        MenuItem::Redo.into(),
        MenuItem::Separator.into(),
        MenuItem::Cut.into(),
        MenuItem::Copy.into(),
        MenuItem::Paste.into(),
        #[cfg(not(target_os = "macos"))]
          MenuItem::Separator.into(),
        MenuItem::SelectAll.into(),
      ]),
    )),

    #[cfg(not(target_os = "macos"))]
    MenuEntry::Submenu(Submenu::new(
      "File",
      Menu::with_items([
        MenuItem::Quit.into()
      ])
    )),

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
  let ctx = tauri::generate_context!();
  tauri::Builder::default()
    .menu(custom_window_menu(&ctx))
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
    .run(ctx)
    .expect("error while running tauri application");
}
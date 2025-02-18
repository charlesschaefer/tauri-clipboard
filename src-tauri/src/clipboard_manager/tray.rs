use tauri::{
    menu::{Menu, MenuItem, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

use super::{handlers::handle_tray_event, history::ClipboardHistory};

pub fn create_tray_menu(history: &Vec<String>) -> Vec<(String, String)> {
    let mut menu_items = Vec::new();

    // Add clipboard history items
    for (index, item) in history.iter().enumerate() {
        let display_text = if item.len() > 30 {
            format!("{}...", &item[..30])
        } else {
            item.clone()
        };

        menu_items.push((format!("item_{}", index), display_text));
    }

    menu_items
}

pub fn setup_tray_menu(app_handle: &AppHandle, update_tray: Option<bool>) {
    let menu = Menu::new(app_handle).unwrap();
    let items: Vec<String>;
    if let Some(_) = update_tray {
        items = app_handle.state::<ClipboardHistory>().get_items();
    } else {
        items = Vec::new();
    }

    // Create initial empty menu
    let menu_items = create_tray_menu(&items);

    for (id, text) in menu_items {
        //let item = MenuItem::with_id(app_handle, &id, text, true, None::<&str>)?;
        let item = MenuItemBuilder::new(text)
            .enabled(true)
            .id(id)
            .build(app_handle)
            .unwrap();
        menu.append(&item).unwrap();
    }

    // Add separator and quit
    let quit_item = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>).unwrap();
    let show_item = MenuItem::with_id(app_handle, "show", "Settings", true, None::<&str>).unwrap();
    //menu_items_vec.push(MenuItem::Separator(app_handle));
    menu.append(&show_item).unwrap();
    menu.append(&quit_item).unwrap();

    if let Some(_) = update_tray {
        // Update the tray menu
        if let Some(tray) = app_handle.tray_by_id("main") {
            let _ = tray.set_menu(Some(menu));
        }
    } else {
        // Create tray icon
        //TrayIconBuilder::new()
        TrayIconBuilder::with_id("main")
            .menu(&menu)
            .on_menu_event(move |app_handle, event| {
                handle_tray_event(app_handle, event);
            })
            .icon(app_handle.default_window_icon().unwrap().clone())
            .build(app_handle)
            .unwrap();
    }

    //// Sets up the event that prevents the window from closing and hides it instead
    let window = app_handle.get_webview_window("main").unwrap();
    let window_hider = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            window_hider.hide().unwrap();
        }
    });
}

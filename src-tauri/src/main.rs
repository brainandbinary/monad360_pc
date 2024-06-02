// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::thread;

use queries::Score;
use rusqlite::Connection;
use tauri::{ipc::RemoteDomainAccessScope, Manager};
use tauri::{CustomMenuItem, Menu, MenuEvent, MenuItem, Submenu, WindowUrl};

pub mod db;
pub mod queries;
mod ui_commands;
use db::{DBError, Database, DB};
use uuid::Uuid;

fn open_window(event: tauri::WindowMenuEvent) {
    /* TO AVOID DUPLICATE LABLE ERROR */
    let UUID = Uuid::new_v4().to_string();
    match event.menu_item_id() {
        "verbal" => {
            let app_handle = event.window().app_handle();

           

            tauri::WindowBuilder::new(
                &app_handle,
                &UUID, /* the unique window label */
                tauri::WindowUrl::App("chart.html".into()),
            )
            .menu(Menu::new())
            .title("Verbal analysis")
            .always_on_top(true)
            .content_protected(true)
            .build()
            .unwrap();
        }
        "quant"  => {
            let app_handle = event.window().app_handle();

            tauri::WindowBuilder::new(
                &app_handle,
                &UUID, /* the unique window label */
                tauri::WindowUrl::App("chart_quant.html".into()),
            )
            .menu(Menu::new())
            .title("Quant analysis")
            .always_on_top(true)
            .content_protected(true)
            .build()
            .unwrap();
        },

        _  => {
            let app_handle = event.window().app_handle();

            tauri::WindowBuilder::new(
                &app_handle,
                &UUID, /* the unique window label */
                tauri::WindowUrl::App("verbal_erro_chart.html".into()),
            ).inner_size(400.0, 600.0)
            .menu(Menu::new())
            .title("Vebal error analysis")
            .always_on_top(true)
            .content_protected(true)
            .build()
            .unwrap();
        }
    }
}


fn resize_or_close(event: tauri::WindowMenuEvent) {
    match event.menu_item_id() {
        "close" => {
            event.window().close().unwrap();
        }
        "mini_maximize" => {
            
            match event.window().is_fullscreen() {
                Err(_) => {},
                Ok(_) => {
                    event.window().set_fullscreen(false).unwrap();
                }
            }
        }
        _ => {}
    }
}


fn main() -> Result<(), DBError> {
    queries::create_mock_score_table();
    queries::create_verbal_ana_table();
    queries::create_verbal_error_table();


    let quant = CustomMenuItem::new("quant".to_string(), "Quant");
    let verbal = CustomMenuItem::new("verbal".to_string(), "Verbal");
    let v_errors = CustomMenuItem::new("v_error".to_string(), "Verbal Errors");
    let analysis = Submenu::new("Analysis", Menu::new().add_item(quant).add_item(verbal).add_item(v_errors));
   
   
    let mini_maximize = CustomMenuItem::new("mini_maximize".to_string(), "Normal screen");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let app_manu = Submenu::new("App", Menu::new().add_item(mini_maximize).add_item(close));

    let menu = Menu::new().add_submenu(analysis)
    .add_submenu(app_manu);

    tauri::Builder::default()
        .menu(menu)
        .setup(|app: &mut tauri::App| {
            app.ipc_scope().configure_remote_access(
                RemoteDomainAccessScope::new("monad360.com")
                    .add_window("main")
                    .enable_tauri_api(),
            );

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ui_commands::save_scores, 
            ui_commands::get_scores, 
            ui_commands::get_verbal_anas,
            ui_commands::get_verbal_errors,
            ui_commands::insert_or_update_verbal_errors
            ])
        .on_menu_event(resize_or_close)
        .on_menu_event(open_window)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

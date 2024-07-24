// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::{ipc::RemoteDomainAccessScope, Manager};
use tauri::{CustomMenuItem, Menu, Submenu};

pub mod host;
pub mod db;
pub mod queries;
mod ui_commands;
use db::{DBError};
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

        "v_error"  => {
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

        "app_id"  => {
            let app_handle = event.window().app_handle();

            tauri::WindowBuilder::new(
                &app_handle,
                &UUID, /* the unique window label */
                tauri::WindowUrl::App("app_id.html".into()),
            )
            .menu(Menu::new())
            .title("App Id")
            .build()
            .unwrap();
        }
        _ => {}
    }
}


fn resize_or_close(event: tauri::WindowMenuEvent) {
    match event.menu_item_id() {
        "close" => {
            event.window().close().unwrap();
        }
        "screen" => {
            
        
           
            if event.window().is_fullscreen().unwrap_or(false) {
                event.window().set_fullscreen(false).unwrap();
                
            } else {
                event.window().set_fullscreen(true).unwrap();
                
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
    let gre = Submenu::new("GRE", Menu::new().add_item(quant).add_item(verbal).add_item(v_errors));
   
   
    let screen = CustomMenuItem::new("screen".to_string(), "Toggle screen");
    let app_id: CustomMenuItem = CustomMenuItem::new("app_id".to_string(), "App Id");
    
    let close = CustomMenuItem::new("close".to_string(), "Close");
    
    let app_manu = Submenu::new("App", Menu::new().add_item(screen).add_item(app_id).add_item(close));

    let menu = Menu::new()
    .add_submenu(app_manu).add_submenu(gre);

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
            ui_commands::app_id, 
            ui_commands::save_scores, 
            ui_commands::get_scores, 
            ui_commands::get_verbal_anas,
            ui_commands::get_verbal_errors,
            ui_commands::insert_or_update_verbal_errors,
            ui_commands::set_fullscreen,
            ])
        .on_menu_event(resize_or_close)
        .on_menu_event(open_window)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}




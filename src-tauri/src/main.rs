// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, PhysicalPosition, PhysicalSize, RunEvent, WebviewUrl, WindowEvent};

const WIDTH: f64 = 800.;
const HEIGHT: f64 = 600.;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = tauri::window::WindowBuilder::new(app, "main")
                .inner_size(WIDTH, HEIGHT)
                .build()?;

            let _webview = window.add_child(
                tauri::webview::WebviewBuilder::new(
                    "main",
                    WebviewUrl::External("https://github.com/tauri-apps/tauri".parse().unwrap()),
                ),
                // PhysicalPosition::new(0., 0.),
                PhysicalPosition::new(1., 1.),
                PhysicalSize::new(WIDTH / 2., HEIGHT / 2.),
            )?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, event| {
            match event {
                RunEvent::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(_size) => {
                        let Some(webview) = app_handle.get_webview("main") else {
                            return;
                        };

                        // let _ = webview.set_position(PhysicalPosition::new(1., 1.));
                        let _ = webview.set_size(PhysicalSize::new(WIDTH / 2., HEIGHT / 2.));
                    }
                    _ => {}
                },
                _ => {}
            };
        });
}

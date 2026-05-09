use android_emulator::{
    proto::{image_format::ImgFormat, ImageFormat},
    EmulatorConfig, GrpcAuthConfig,
};
use base64::{engine::general_purpose, Engine as _};
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Runtime,
};
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Avd {
    name: String,
}

#[derive(Serialize, Clone)]
struct FrameEvent {
    data: String,
}

#[tauri::command]
async fn get_avds() -> Result<Vec<Avd>, String> {
    let output = std::process::Command::new("emulator")
        .arg("-list-avds")
        .output()
        .map_err(|e| format!("Failed to execute emulator command: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let avds = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|name| Avd {
            name: name.to_string(),
        })
        .collect();

    Ok(avds)
}

#[tauri::command]
async fn launch_emulator<R: Runtime>(
    app: tauri::AppHandle<R>,
    name: String,
    mode: String,
) -> Result<(), String> {
    let mut extra_args = vec![
        "-read-only".to_string(),
        "-no-snapshot-save".to_string(),
        "-no-audio".to_string(),
        "-gpu".to_string(),
        "host".to_string(),
        "-memory".to_string(),
        "4096".to_string(),
        // "-serial".to_string(), "GUI_EMU".to_string(),
    ];
    let mut no_snapshot_load = false;

    match mode.as_str() {
        "cold" => no_snapshot_load = true,
        "wipe" => extra_args.push("-wipe-data".to_string()),
        _ => {}
    }

    let config = EmulatorConfig::new(&name)
        .with_window(false) // Headless for embedding
        .with_grpc_auth(GrpcAuthConfig::None)
        .with_snapshot_load(!no_snapshot_load)
        .with_extra_args(extra_args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    println!("Spawning emulator: {}", name);
    let emulator = config.spawn().await.map_err(|e| {
        eprintln!("Failed to spawn emulator: {}", e);
        e.to_string()
    })?;

    let _ = Notification::new()
        .summary("Emulator Starting")
        .body(&format!("{} is booting up in embedded mode...", name))
        .show();

    // Start streaming in a background task
    let app_clone = app.clone();
    tokio::spawn(async move {
        println!("Background task started, connecting to gRPC...");
        // Connect to the emulator's gRPC controller
        match emulator.connect(Some(Duration::from_secs(60)), true).await {
            Ok(mut client) => {
                println!("Connected to gRPC. Waiting for boot...");
                // Store the client in app state or a global if needed for interactions
                // For now we just stream
                let _ = client
                    .wait_until_booted(Duration::from_secs(300), None)
                    .await;

                println!("Emulator booted. Starting stream...");
                let request = ImageFormat {
                    format: ImgFormat::Png as i32,
                    width: 450,
                    height: 800,
                    ..Default::default()
                };

                match client.protocol_mut().stream_screenshot(request).await {
                    Ok(response) => {
                        let mut stream = response.into_inner();
                        println!("Stream established.");
                        while let Some(Ok(frame)) = stream.next().await {
                            let b64 = general_purpose::STANDARD.encode(&frame.image);
                            let _ = app_clone.emit("emulator-frame", FrameEvent { data: b64 });
                        }
                    }
                    Err(e) => eprintln!("Failed to stream screenshot: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to connect to gRPC: {}", e),
        }
    });

    Ok(())
}

#[tauri::command]
async fn send_mouse_event(x: i32, y: i32, buttons: i32) -> Result<(), String> {
    use android_emulator::proto::MouseEvent;

    let emulators = android_emulator::list_emulators()
        .await
        .map_err(|e| e.to_string())?;
    if let Some(emulator) = emulators.iter().find(|e| e.serial() == "GUI_EMU") {
        let mut client = emulator
            .connect(Some(Duration::from_secs(5)), true)
            .await
            .map_err(|e| e.to_string())?;

        // Get physical dimensions from metadata
        let lcd_w: i32 = emulator
            .get_metadata("lcd.width")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1440);
        let lcd_h: i32 = emulator
            .get_metadata("lcd.height")
            .and_then(|s| s.parse().ok())
            .unwrap_or(3120);

        // Map from 450x800 (our requested stream size) to physical size
        let scaled_x = (x * lcd_w) / 450;
        let scaled_y = (y * lcd_h) / 800;

        let req = MouseEvent {
            x: scaled_x,
            y: scaled_y,
            buttons,
            ..Default::default()
        };

        client
            .protocol_mut()
            .send_mouse(req)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn send_key(key: String) -> Result<(), String> {
    use android_emulator::proto::KeyboardEvent;

    // We need to find the running emulator and connect to it
    let emulators = android_emulator::list_emulators()
        .await
        .map_err(|e| e.to_string())?;
    if let Some(emulator) = emulators.iter().find(|e| e.serial() == "GUI_EMU") {
        let mut client = emulator
            .connect(Some(Duration::from_secs(5)), true)
            .await
            .map_err(|e| e.to_string())?;

        let key_code = match key.as_str() {
            "home" => "Home",
            "back" => "Back",
            "recent" => "AppSwitch",
            "power" => "Power",
            _ => return Ok(()),
        };

        let req = KeyboardEvent {
            key: key_code.to_string(),
            ..Default::default()
        };

        client
            .protocol_mut()
            .send_key(req)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_avds,
            launch_emulator,
            send_key,
            send_mouse_event
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

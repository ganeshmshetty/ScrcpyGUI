use tauri::{command, api::process::Command, Manager};

#[command]
async fn launch_scrcpy(app: tauri::AppHandle, args: Vec<String>) -> Result<(), String> {
    // use the sidecar name (no "./")
    let (mut rx, _child) = Command::new_sidecar("scrcpy")
        .map_err(|e| e.to_string())?
        .args(args)
        .spawn()
        .map_err(|e| e.to_string())?;

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            // send events to all windows (or pick one)
            let _ = app.emit_all("scrcpy-event", format!("{:?}", event));
        }
    });

    Ok(())
}

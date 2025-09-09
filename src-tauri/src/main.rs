use tauri::api::process::Command;

fn main() {
    tauri::async_runtime::block_on(async {
        let (mut rx, _child) = Command::new_sidecar("scrcpy")
            .expect("Failed to find scrcpy sidecar")
            .args(["--help"]) // safer test: prints usage instead of failing with no device
            .spawn()
            .expect("Failed to launch scrcpy");

        while let Some(event) = rx.recv().await {
            println!("SCRCPY event: {:?}", event);
        }
    });
}

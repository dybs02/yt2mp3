use tauri::api::process::CommandEvent;
use tauri::api::process::Command;


#[tauri::command]
pub fn download(link: &str, window: tauri::Window, handle: tauri::AppHandle) {
  let download_path = get_download_save_path();
  let ffmpeg_path = get_ffmpeg_path(handle);

  let (mut rx, _) = Command::new_sidecar("yt-dlp")
    .expect("Failed to create `yt-dlp` binary command")
    .args([
      "--paths", &download_path,
      "--ffmpeg-location", &ffmpeg_path,
      "--extract-audio",
      "--audio-format", "mp3",
      "--verbose",
      link
      ])
    .spawn()
    .expect("Failed to spawn sidecar - yt-dlp");

  tauri::async_runtime::spawn(async move {
    while let Some(event) = rx.recv().await {
      if let CommandEvent::Stdout(line) = event {
        window
          .emit("ytdlp", Some(format!("'{}'", line)))
          .expect("failed to emit event");
        println!("{}", line);
      }
    }
  });
}

fn get_ffmpeg_path(handle: tauri::AppHandle) -> String {
  let resource_path = handle.path_resolver()
    .resolve_resource("ffmpeg")
    .expect("failed to resolve resource");

  if cfg!(windows) {
    return resource_path.display().to_string() + ".exe";
  } else if cfg!(unix) {
    return resource_path.display().to_string();
  } else if cfg!(target_os = "macos") {
    return resource_path.display().to_string(); 
  } else {
    panic!("Unsupported OS");
  }
}

fn get_download_save_path() -> String {
  let download_path = dirs_next::download_dir().unwrap().display().to_string();
  return download_path;
}

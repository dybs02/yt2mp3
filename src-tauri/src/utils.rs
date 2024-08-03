use image_base64;

#[tauri::command]
pub fn get_image_b64(path: &str) -> String {
  return image_base64::to_base64(path);
}

use crate::context::CapturedContext;
use tauri::{AppHandle, Manager, PhysicalPosition};

pub async fn open_palette(app: &AppHandle, context: CapturedContext) -> Result<(), String> {
    if let Some(palette) = app.get_window("palette") {
        if let Ok(Some(monitor)) = palette.current_monitor() {
            let size = monitor.size();
            let x = (size.width as i32 / 2) - 280;
            let y = (size.height as f64 * 0.38) as i32;
            let _ = palette.set_position(PhysicalPosition::new(x, y));
        }

        let _ = palette.emit("palette-context", &context);
        let _ = palette.show();
        let _ = palette.set_focus();
        Ok(())
    } else {
        Err("Palette window 'palette' not found in Tauri app manager".to_string())
    }
}

pub fn close_palette(app: &AppHandle) -> Result<(), String> {
    if let Some(palette) = app.get_window("palette") {
        let _ = palette.hide();
        Ok(())
    } else {
        Err("Palette window 'palette' not found in Tauri app manager".to_string())
    }
}

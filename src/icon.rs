use image::GenericImageView;

pub fn load_tray_icon() -> Option<tray_icon::Icon> {
    #[cfg(target_os = "windows")]
    const ICON: &[u8] = include_bytes!("../icons/tray.ico");

    #[cfg(not(target_os = "windows"))]
    const ICON: &[u8] = include_bytes!("../icons/tray.png");

    let img = image::load_from_memory(ICON).ok()?;
    let (width, height) = img.dimensions();

    tray_icon::Icon::from_rgba(img.to_rgba8().into_raw(), width, height).ok()
}

pub fn load_window_icon() -> Option<tao::window::Icon> {
    #[cfg(target_os = "windows")]
    const ICON: &[u8] = include_bytes!("../icons/icon.ico");

    #[cfg(not(target_os = "windows"))]
    const ICON: &[u8] = include_bytes!("../icons/icon.png");

    let img = image::load_from_memory(ICON).ok()?;
    let (width, height) = img.dimensions();

    tao::window::Icon::from_rgba(img.to_rgba8().into_raw(), width, height).ok()
}

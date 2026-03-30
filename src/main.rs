#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use muda::{Menu, MenuEvent, MenuItem};
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

mod icon;
mod window;

use crate::{icon::load_tray_icon, window::WindowBuilder};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let win_handle = WindowBuilder.build_window(&event_loop);

    let _webview = WindowBuilder.build_webview(win_handle.clone());

    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("Quit", true, None);
    tray_menu.append(&quit_item).unwrap();
    let quit_id = quit_item.id().clone();

    let _tray = TrayIconBuilder::new()
        .with_icon(load_tray_icon().expect("Failed to load icon"))
        .with_tooltip("Discord")
        .with_menu(Box::new(tray_menu))
        .build()
        .expect("Failed to build tray icon");

    let tray_channel = TrayIconEvent::receiver();
    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(tray_event) = tray_channel.try_recv()
            && let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = tray_event
        {
            if !win_handle.is_visible() {
                win_handle.set_visible(true);
            } else {
                win_handle.set_focus();
            }
        }

        if let Ok(menu_event) = menu_channel.try_recv()
            && menu_event.id == quit_id
        {
            std::process::exit(0);
        }

        if let Event::WindowEvent { event, .. } = event
            && event == WindowEvent::CloseRequested
        {
            win_handle.set_visible(false);
        }
    });
}

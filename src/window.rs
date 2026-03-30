use std::sync::Arc;

use tao::platform::unix::WindowExtUnix;
use tao::{event_loop::EventLoop, window::Window};
use wry::WebViewBuilder;
use wry::WebViewBuilderExtUnix;

use crate::icon::load_window_icon;

#[derive(Default)]
pub struct WindowBuilder;

impl WindowBuilder {
    pub fn build_window<T>(&self, event_loop: &EventLoop<T>) -> Arc<Window> {
        let window = tao::window::WindowBuilder::new()
            .with_title("Discord")
            .with_visible(false)
            .with_window_icon(load_window_icon())
            .with_inner_size(tao::dpi::LogicalSize::new(800.0, 600.0))
            .build(event_loop)
            .expect("Failed to build window");

        Arc::new(window)
    }

    pub fn build_webview(&self, window: Arc<Window>) -> wry::WebView {
        let webview = self.create_webview(window.clone());

        self.finish_webview(&window.clone(), webview)
    }

    fn create_webview(&self, window: Arc<Window>) -> WebViewBuilder<'_> {
        WebViewBuilder::new()
            .with_url("https://discord.com/app")
            .with_initialization_script(
                "window.addEventListener('load',_=>window.ipc.postMessage('loaded'))",
            )
            .with_ipc_handler(move |req| {
                if req.body() == "loaded" {
                    window.set_visible(true);
                }
            })
            .with_devtools(cfg!(debug_assertions))
    }

    #[cfg(target_os = "linux")]
    fn finish_webview(&self, window: &Window, webview: WebViewBuilder<'_>) -> wry::WebView {
        let vbox = window.default_vbox().expect("Failed to get vbox");
        webview.build_gtk(vbox).expect("Failed to build WebView")
    }

    #[cfg(not(target_os = "linux"))]
    fn finish_webview(&self, window: &Window, webview: WebViewBuilder<'_>) -> wry::WebView {
        webview.build(&window).expect("Failed to build WebView")
    }
}

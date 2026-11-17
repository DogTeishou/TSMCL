use gpui_kit::{
    AppContext, Bounds, ParentElement, Render, WindowBounds, WindowOptions, div, open_window, px, size};

pub mod home;
pub mod sidebar;

use sidebar::Sidebar;

struct MainWindow;

impl Render for MainWindow {
    fn render(&mut self, _: &mut gpui_kit::Window, cx: &mut gpui_kit::prelude::Context<Self>) -> impl gpui_kit::prelude::IntoElement {
        div()
            .child(cx.new(|_| Sidebar))
    }
}

#[tokio::main]
async fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);

            let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
            
            open_window(WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(400.0), px(300.0))),
                .. WindowOptions::default()
            }, cx, |window, cx| {
                window.set_window_title("TSMCL");
                cx.new(|_| MainWindow)
            }).expect("open window failed");
        });
}
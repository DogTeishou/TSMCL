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

            let display = cx.primary_display().expect("Can not find display");
            let screen_size = display.bounds().size;
            let window_size = size(
                screen_size.width * 0.42,
                screen_size.height * 0.48,
            );
            let bounds = Bounds::centered(None, window_size, cx);
                       
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
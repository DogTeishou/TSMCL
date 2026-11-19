use std::borrow::Cow;

use gpui_kit::{
    AnyView, AppContext, AssetSource, Bounds, Context, Entity, IntoElement, ParentElement, Render,
    Result, SharedString, Styled, Window, WindowBounds, WindowOptions, open_window, px, size,
};
use gpui_kit::component::{ActiveTheme, h_flex};

use crate::about::About;
use crate::accounts::Accounts;
use crate::download::Download;
use crate::home::Home;
use crate::settings::Settings;
use crate::sidebar::{Page, sidebar};

pub mod about;
pub mod accounts;
pub mod download;
pub mod home;
pub mod settings;
pub mod sidebar;

// Icons used by TSMCL that are not part of the default gpui-kit bundle.
gpui_kit::assets::icon_assets!(
    ExtraIcons,
    [
        HardDriveDownload, Wrench, Gamepad2, Box, FlaskConical, Archive, PartyPopper, Anvil,
        Hammer, Scissors, Layers, Rocket, Coffee, Palette,
        Shirt, Trash, UserPlus, ShieldCheck, Ban, Shield, Scale, Copyright,
    ]
);

struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = ExtraIcons.load(path)? {
            return Ok(Some(bytes));
        }
        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut paths = gpui_kit::assets::Assets.list(path)?;
        paths.extend(ExtraIcons.list(path)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}

/// Root view: the navigation sidebar on the left and the current page on the right.
///
/// Every page is created once and kept alive, so switching back and forth keeps its state.
pub struct MainWindow {
    page: Page,
    home: Entity<Home>,
    download: Entity<Download>,
    settings: Entity<Settings>,
    about: Entity<About>,
    accounts: Entity<Accounts>,
}

impl MainWindow {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            page: Page::Launch,
            home: cx.new(|_| Home),
            download: cx.new(|_| Download),
            settings: cx.new(|_| Settings),
            about: cx.new(|_| About),
            accounts: cx.new(|_| Accounts),
        }
    }

    pub fn set_page(&mut self, page: Page, cx: &mut Context<Self>) {
        if self.page != page {
            self.page = page;
            cx.notify();
        }
    }

    fn current_page(&self) -> AnyView {
        match self.page {
            Page::Launch => self.home.clone().into(),
            Page::Download => self.download.clone().into(),
            Page::Settings => self.settings.clone().into(),
            Page::About => self.about.clone().into(),
            Page::Accounts => self.accounts.clone().into(),
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(sidebar(self.page, cx))
            .child(h_flex().flex_1().h_full().min_w_0().child(self.current_page()))
    }
}

#[tokio::main]
async fn main() {
    gpui_kit::application()
        .with_assets(AppAssets)
        .run(|cx| {
            gpui_kit::init(cx);

            let display = cx.primary_display().expect("Can not find display");
            let screen_size = display.bounds().size;
            let window_size = size(screen_size.width * 0.5, screen_size.height * 0.56);
            let bounds = Bounds::centered(None, window_size, cx);

            open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_min_size: Some(size(px(860.0), px(540.0))),
                    ..WindowOptions::default()
                },
                cx,
                |window, cx| {
                    window.set_window_title("TSMCL");
                    cx.new(MainWindow::new)
                },
            )
            .expect("open window failed");
        });
}

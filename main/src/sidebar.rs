use gpui_kit::{
    Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Styled, assets::IconName,
    div, px,
};
use crate::MainWindow;
use gpui_kit::component::{
    ActiveTheme, Icon, Sizable, StyledExt, h_flex, v_flex,
    avatar::Avatar,
    sidebar::{Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem},
};

/// Top-level pages shown in the navigation sidebar.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Launch,
    Download,
    Settings,
    About,
    Accounts,
}

/// The left navigation sidebar, with the current account in the footer.
/// Clicking an item (or the account footer) switches the page shown by [`MainWindow`].
pub fn sidebar(active: Page, cx: &mut Context<MainWindow>) -> impl IntoElement {
    let item = |label: &'static str, icon: IconName, page: Page, cx: &mut Context<MainWindow>| {
        SidebarMenuItem::new(label)
            .icon(icon)
            .active(active == page)
            .on_click(cx.listener(move |this, _, _, cx| this.set_page(page, cx)))
    };

    Sidebar::new("sidebar")
        .w(px(232.0))
        .header(
            SidebarHeader::new().child(
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .size(px(32.0))
                            .rounded(cx.theme().radius)
                            .bg(cx.theme().primary)
                            .text_color(cx.theme().primary_foreground)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Icon::new(IconName::Gamepad2).small()),
                    )
                    .child(
                        v_flex()
                            .child(div().font_semibold().child("TSMCL"))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("第三方启动器"),
                            ),
                    ),
            ),
        )
        .child(
            SidebarGroup::new("导航").child(
                SidebarMenu::new()
                    .child(item("启动", IconName::Play, Page::Launch, cx))
                    .child(item("下载", IconName::HardDriveDownload, Page::Download, cx))
                    .child(item("设置", IconName::Settings, Page::Settings, cx))
                    .child(item("关于", IconName::Info, Page::About, cx)),
            ),
        )
        .child(
            SidebarGroup::new("账号").child(
                SidebarMenu::new().child(item("账号管理", IconName::User, Page::Accounts, cx)),
            ),
        )
        .footer(
            SidebarFooter::new()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| this.set_page(Page::Accounts, cx)),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .child(Avatar::new().name("Player").small())
                        .child(
                            v_flex()
                                .child(div().text_sm().child("Player"))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("正版验证"),
                                ),
                        ),
                )
                .child(Icon::new(IconName::ChevronsUpDown).small()),
        )
}

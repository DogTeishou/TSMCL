use gpui_kit::{
    App, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName, div, px,
};
use gpui_kit::component::{
    ActiveTheme, Icon, Sizable, StyledExt, h_flex, v_flex,
    scroll::ScrollableElement,
    sidebar::{Sidebar, SidebarCollapsible, SidebarGroup, SidebarMenu, SidebarMenuItem},
};

use crate::home::card;

/// Download page: pick a game version or a mod loader to install.
pub struct Download;

impl Render for Download {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(category_sidebar())
            .child(
                v_flex()
                    .id("download-content")
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .p_6()
                    .gap_4()
                    .overflow_y_scrollbar()
                    .child(latest_card(cx))
                    .child(version_group("正式版", 112, IconName::Box, cx))
                    .child(version_group("预览版", 830, IconName::FlaskConical, cx))
                    .child(version_group("远古版", 165, IconName::Archive, cx))
                    .child(version_group("愚人节版", 12, IconName::PartyPopper, cx)),
            )
    }
}

/// Secondary sidebar listing what can be downloaded.
fn category_sidebar() -> impl IntoElement {
    Sidebar::new("download-categories")
        .w(px(180.0))
        .collapsible(SidebarCollapsible::None)
        .child(
            SidebarGroup::new("游戏").child(
                SidebarMenu::new()
                    .child(SidebarMenuItem::new("Minecraft").icon(IconName::Box).active(true)),
            ),
        )
        .child(
            SidebarGroup::new("加载器").child(
                SidebarMenu::new()
                    .child(SidebarMenuItem::new("Forge").icon(IconName::Anvil))
                    .child(SidebarMenuItem::new("NeoForge").icon(IconName::Hammer))
                    .child(SidebarMenuItem::new("Fabric").icon(IconName::Scissors))
                    .child(SidebarMenuItem::new("Quilt").icon(IconName::Layers)),
            ),
        )
}

/// The newest release and snapshot.
fn latest_card(cx: &App) -> impl IntoElement {
    card(cx)
        .gap_3()
        .child(div().font_semibold().child("最新版本"))
        .child(version_row(IconName::Box, "26.3", "最新正式版，发布于 2026/09/15 19:23", cx))
        .child(version_row(
            IconName::FlaskConical,
            "26.4-snapshot-10",
            "最新预览版，发布于 2026/11/18 00:02",
            cx,
        ))
}

fn version_row(
    icon: IconName,
    name: &'static str,
    detail: &'static str,
    cx: &App,
) -> impl IntoElement {
    h_flex()
        .gap_3()
        .px_2()
        .py_1p5()
        .rounded(cx.theme().radius)
        .hover(|this| this.bg(cx.theme().accent))
        .child(
            div()
                .size(px(32.0))
                .flex_shrink_0()
                .rounded(cx.theme().radius)
                .bg(cx.theme().muted)
                .flex()
                .items_center()
                .justify_center()
                .child(Icon::new(icon).small()),
        )
        .child(
            v_flex()
                .min_w_0()
                .child(div().text_sm().child(name))
                .child(div().text_xs().text_color(cx.theme().muted_foreground).child(detail)),
        )
}

/// A collapsed group of versions; the header shows the type and count.
fn version_group(title: &'static str, count: usize, icon: IconName, cx: &App) -> impl IntoElement {
    card(cx).child(
        h_flex()
            .gap_2()
            .child(Icon::new(icon).small().text_color(cx.theme().muted_foreground))
            .child(div().flex_1().font_semibold().child(format!("{title} ({count})")))
            .child(Icon::new(IconName::ChevronDown).small().text_color(cx.theme().muted_foreground)),
    )
}

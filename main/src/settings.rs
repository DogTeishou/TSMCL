use gpui_kit::{
    App, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName, div,
    prelude::FluentBuilder, px, relative,
};
use gpui_kit::component::{
    ActiveTheme, Icon, Sizable, StyledExt, h_flex, v_flex,
    radio::Radio,
    scroll::ScrollableElement,
    sidebar::{Sidebar, SidebarCollapsible, SidebarGroup, SidebarMenu, SidebarMenuItem},
};

use crate::home::card;

/// Settings page. Controls are drawn statically; nothing is wired up yet.
pub struct Settings;

impl Render for Settings {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(category_sidebar())
            .child(
                v_flex()
                    .id("settings-content")
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .p_6()
                    .gap_4()
                    .overflow_y_scrollbar()
                    .child(launch_options(cx))
                    .child(memory_options(cx))
                    .child(java_options(cx)),
            )
    }
}

fn category_sidebar() -> impl IntoElement {
    Sidebar::new("settings-categories")
        .w(px(180.0))
        .collapsible(SidebarCollapsible::None)
        .child(
            SidebarGroup::new("游戏").child(
                SidebarMenu::new()
                    .child(SidebarMenuItem::new("启动").icon(IconName::Rocket).active(true))
                    .child(SidebarMenuItem::new("Java").icon(IconName::Coffee)),
            ),
        )
        .child(
            SidebarGroup::new("启动器").child(
                SidebarMenu::new().child(SidebarMenuItem::new("个性化").icon(IconName::Palette)),
            ),
        )
}

fn section(title: &'static str, cx: &App) -> gpui_kit::Div {
    card(cx)
        .p_5()
        .gap_3()
        .child(div().font_semibold().text_color(cx.theme().primary).child(title))
}

/// A label on the left and a control on the right, like a form row.
fn row(label: &'static str, control: impl IntoElement, cx: &App) -> impl IntoElement {
    h_flex()
        .gap_4()
        .child(
            div()
                .w(px(120.0))
                .flex_shrink_0()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(label),
        )
        .child(div().flex_1().min_w_0().child(control))
}

/// Looks like a select box. `disabled` greys it out for options that cannot be changed.
fn fake_select(value: &'static str, disabled: bool, cx: &App) -> impl IntoElement {
    h_flex()
        .h(px(32.0))
        .px_3()
        .gap_2()
        .rounded(cx.theme().radius)
        .border_1()
        .border_color(cx.theme().input)
        .when(disabled, |this| this.bg(cx.theme().muted))
        .child(
            div()
                .flex_1()
                .text_sm()
                .when(disabled, |this| this.text_color(cx.theme().muted_foreground))
                .child(value),
        )
        .child(Icon::new(IconName::ChevronDown).small().text_color(cx.theme().muted_foreground))
}

/// Looks like a text input.
fn fake_input(value: &'static str, cx: &App) -> impl IntoElement {
    h_flex()
        .h(px(32.0))
        .px_3()
        .rounded(cx.theme().radius)
        .border_1()
        .border_color(cx.theme().input)
        .text_sm()
        .child(value)
}

/// Looks like a slider at `fraction` (0.0 – 1.0).
fn fake_slider(fraction: f32, cx: &App) -> impl IntoElement {
    div()
        .relative()
        .h(px(16.0))
        .flex()
        .items_center()
        .child(
            div()
                .w_full()
                .h(px(4.0))
                .rounded_full()
                .bg(cx.theme().muted)
                .child(div().h_full().w(relative(fraction)).rounded_full().bg(cx.theme().primary)),
        )
        .child(
            div()
                .absolute()
                .left(relative(fraction))
                .ml(px(-8.0))
                .size(px(16.0))
                .rounded_full()
                .border_2()
                .border_color(cx.theme().primary)
                .bg(cx.theme().background),
        )
}

fn launch_options(cx: &App) -> impl IntoElement {
    section("启动选项", cx)
        .child(row("默认版本隔离", fake_select("隔离所有实例", false, cx), cx))
        .child(row("游戏窗口标题", fake_select("默认", false, cx), cx))
        .child(row("自定义信息", fake_input("TSMCL", cx), cx))
        .child(row("启动器可见性", fake_select("游戏启动后仍保持不变", false, cx), cx))
        .child(row("窗口大小", fake_select("默认", false, cx), cx))
        .child(row("游戏语言", fake_select("简体中文", false, cx), cx))
        .child(row("正版验证方式", fake_select("Microsoft 设备代码登录", true, cx), cx))
}

fn memory_options(cx: &App) -> impl IntoElement {
    section("游戏内存", cx)
        .child(Radio::new("memory-auto").label("自动配置").checked(true))
        .child(
            h_flex()
                .gap_4()
                .child(Radio::new("memory-custom").label("自定义"))
                .child(div().flex_1().child(fake_slider(0.35, cx)))
                .child(div().w(px(56.0)).text_sm().text_right().child("4.0 GB")),
        )
        .child(
            div()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child("系统内存 16.0 GB，已使用 9.2 GB"),
        )
}

fn java_options(cx: &App) -> impl IntoElement {
    section("Java", cx)
        .child(row("Java 路径", fake_select("自动选择合适的 Java", false, cx), cx))
        .child(row("JVM 参数", fake_input("-XX:+UseG1GC", cx), cx))
}

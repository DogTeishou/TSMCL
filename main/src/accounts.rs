use gpui_kit::{
    App, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName, div, prelude::FluentBuilder, px,
};
use gpui_kit::component::{
    ActiveTheme, Icon, Sizable, StyledExt, h_flex, v_flex,
    avatar::Avatar,
    button::{Button, ButtonVariants},
    scroll::ScrollableElement,
    tag::Tag,
};

use crate::home::card;

/// Account page: current account with skin / cape, the saved account list and Microsoft sign-in.
/// Layout only; all data is placeholder.
pub struct Accounts;

impl Render for Accounts {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(
                h_flex()
                    .id("accounts-content")
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .p_6()
                    .gap_5()
                    .items_start()
                    .overflow_y_scrollbar()
                    .child(current_account(cx))
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w_0()
                            .gap_5()
                            .child(account_list(cx))
                            .child(sign_in(cx)),
                    ),
            )
    }
}

/// Left column: the selected account, a skin preview and the capes it owns.
fn current_account(cx: &App) -> impl IntoElement {
    card(cx)
        .w(px(280.0))
        .flex_shrink_0()
        .p_0()
        .gap_0()
        .child(
            h_flex()
                .p_4()
                .gap_3()
                .border_b_1()
                .border_color(cx.theme().border)
                .child(Avatar::new().name("Player"))
                .child(
                    v_flex()
                        .flex_1()
                        .min_w_0()
                        .child(div().font_semibold().child("Player"))
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Microsoft 账号"),
                        ),
                )
                .child(Tag::success().small().child("正版验证")),
        )
        .child(
            v_flex()
                .h(px(260.0))
                .items_center()
                .justify_center()
                .gap_2()
                .bg(cx.theme().muted)
                .child(Icon::new(IconName::Shirt).size(px(48.0)).text_color(cx.theme().muted_foreground))
                .child(div().text_sm().text_color(cx.theme().muted_foreground).child("皮肤预览")),
        )
        .child(
            v_flex()
                .p_4()
                .gap_3()
                .border_t_1()
                .border_color(cx.theme().border)
                .child(div().text_sm().font_semibold().child("披风"))
                .child(
                    h_flex()
                        .gap_2()
                        .child(cape_slot(Some(IconName::Ban), true, cx))
                        .child(cape_slot(None, false, cx))
                        .child(cape_slot(None, false, cx)),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("只显示该账号在 Minecraft 官方档案中已拥有的披风。"),
                ),
        )
}

fn cape_slot(icon: Option<IconName>, selected: bool, cx: &App) -> impl IntoElement {
    div()
        .w(px(40.0))
        .h(px(56.0))
        .rounded(cx.theme().radius)
        .border_1()
        .border_color(if selected { cx.theme().primary } else { cx.theme().border })
        .bg(cx.theme().muted)
        .flex()
        .items_center()
        .justify_center()
        .when_some(icon, |this, icon| {
            this.child(Icon::new(icon).small().text_color(cx.theme().muted_foreground))
        })
}

/// Saved accounts, with the current one marked.
fn account_list(cx: &App) -> impl IntoElement {
    card(cx)
        .p_5()
        .gap_3()
        .child(
            h_flex()
                .justify_between()
                .child(div().font_semibold().child("已保存的账号"))
                .child(
                    Button::new("add-account")
                        .primary()
                        .small()
                        .icon(IconName::UserPlus)
                        .label("添加 Microsoft 账号"),
                ),
        )
        .child(account_row("account-0", "Player", true, cx))
        .child(account_row("account-1", "Alex", false, cx))
}

fn account_row(id: &'static str, name: &'static str, current: bool, cx: &App) -> impl IntoElement {
    h_flex()
        .gap_3()
        .p_3()
        .rounded(cx.theme().radius)
        .border_1()
        .border_color(if current { cx.theme().primary } else { cx.theme().border })
        .child(Avatar::new().name(name).small())
        .child(
            v_flex()
                .flex_1()
                .min_w_0()
                .child(div().text_sm().child(name))
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("Microsoft · 正版验证"),
                ),
        )
        .when(current, |this| this.child(Tag::primary().small().child("当前")))
        .when(!current, |this| {
            this.child(Button::new(format!("{id}-use")).outline().small().label("使用"))
        })
        .child(
            Button::new(format!("{id}-remove"))
                .ghost()
                .small()
                .icon(IconName::Trash)
                .tooltip("移除账号"),
        )
}

/// What the device-code sign-in looks like once a code has been requested.
fn sign_in(cx: &App) -> impl IntoElement {
    card(cx)
        .p_5()
        .gap_3()
        .child(
            h_flex()
                .gap_2()
                .child(Icon::new(IconName::ShieldCheck).small().text_color(cx.theme().primary))
                .child(div().font_semibold().child("Microsoft 登录")),
        )
        .child(
            v_flex()
                .gap_1()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child("1. 点击【打开登录页面】，在 Microsoft 官方网页中输入下面的代码")
                .child("2. 使用拥有 Minecraft: Java 版的 Microsoft 账号完成登录")
                .child("3. 登录完成后回到 TSMCL，账号会自动添加"),
        )
        .child(
            h_flex()
                .gap_3()
                .child(
                    div()
                        .flex_1()
                        .py_3()
                        .rounded(cx.theme().radius)
                        .bg(cx.theme().muted)
                        .text_center()
                        .text_2xl()
                        .font_semibold()
                        .child("ABCD-EFGH"),
                )
                .child(Button::new("copy-code").outline().icon(IconName::Copy).label("复制代码"))
                .child(
                    Button::new("open-login")
                        .primary()
                        .icon(IconName::ExternalLink)
                        .label("打开登录页面"),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child("TSMCL 不会接触你的 Microsoft 密码，登录只在 Microsoft 官方网页中进行。"),
        )
}

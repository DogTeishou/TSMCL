use gpui_kit::{
    App, Context, Div, IntoElement, ParentElement, Render, Styled, Window, assets::IconName, div,
    px,
};
use gpui_kit::component::{
    ActiveTheme, Icon, Sizable, StyledExt, h_flex, v_flex,
    avatar::Avatar,
    button::{Button, ButtonVariants},
    tag::Tag,
};


pub struct Home;

impl Render for Home {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(
                v_flex()
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .p_6()
                    .gap_5()
                    .child(page_header(cx))
                    .child(
                        h_flex()
                            .flex_1()
                            .min_h_0()
                            .items_start()
                            .gap_5()
                            .child(launch_panel(cx))
                            .child(
                                v_flex()
                                    .flex_1()
                                    .min_w_0()
                                    .gap_5()
                                    .child(notice_card(cx))
                                    .child(
                                        h_flex()
                                            .gap_4()
                                            .child(stat_card("账号", "1", "Microsoft 账号", cx))
                                            .child(stat_card("实例", "0", "已安装的游戏版本", cx))
                                            .child(stat_card(
                                                "版本",
                                                env!("CARGO_PKG_VERSION"),
                                                "TSMCL",
                                                cx,
                                            )),
                                    ),
                            ),
                    ),
            )
    }
}

pub fn card(cx: &App) -> Div {
    v_flex()
        .p_4()
        .gap_2()
        .rounded(cx.theme().radius_lg)
        .border_1()
        .border_color(cx.theme().border)
        .bg(cx.theme().background)
}

fn page_header(cx: &App) -> impl IntoElement {
    v_flex()
        .gap_1()
        .child(div().text_2xl().font_semibold().child("启动"))
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child("选择账号和游戏实例，然后启动 Minecraft: Java 版。"),
        )
}

/// Account card with the launch button, like the left column of the launcher home page.
fn launch_panel(cx: &App) -> impl IntoElement {
    card(cx)
        .w(px(260.0))
        .flex_shrink_0()
        .p_5()
        .gap_4()
        .child(
            v_flex()
                .items_center()
                .gap_2()
                .py_6()
                .child(Avatar::new().name("Player").with_size(px(72.0)))
                .child(div().text_lg().font_semibold().child("Player"))
                .child(Tag::success().small().child("正版验证")),
        )
        .child(
            v_flex()
                .gap_2()
                .child(
                    Button::new("launch")
                        .primary()
                        .large()
                        .w_full()
                        .icon(IconName::Play)
                        .label("启动游戏"),
                )
                .child(
                    div()
                        .w_full()
                        .text_center()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("未选择实例"),
                ),
        )
        .child(
            h_flex()
                .gap_2()
                .child(Button::new("select-instance").outline().flex_1().label("实例选择"))
                .child(Button::new("instance-settings").outline().flex_1().label("实例设置")),
        )
}

fn notice_card(cx: &App) -> impl IntoElement {
    card(cx)
        .child(
            h_flex()
                .gap_2()
                .child(Icon::new(IconName::Info).small().text_color(cx.theme().primary))
                .child(div().font_semibold().child("关于 TSMCL")),
        )
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(
                    "TSMCL 是独立开发的第三方启动器，不是 Minecraft 官方产品，\
                     未经 Mojang Studios 或 Microsoft 批准，也与其无关。",
                ),
        )
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child("仅支持 Microsoft 正版账号登录，所有游戏文件均从官方服务器下载。"),
        )
}

fn stat_card(title: &'static str, value: &'static str, hint: &'static str, cx: &App) -> impl IntoElement {
    card(cx)
        .flex_1()
        .min_w_0()
        .child(div().text_sm().text_color(cx.theme().muted_foreground).child(title))
        .child(div().text_2xl().font_semibold().child(value))
        .child(div().text_xs().text_color(cx.theme().muted_foreground).child(hint))
}

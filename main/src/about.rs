use gpui_kit::{
    App, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName, div, px,
};
use gpui_kit::component::{
    ActiveTheme, Icon, Sizable, StyledExt, h_flex, v_flex,
    button::{Button, ButtonVariants},
    scroll::ScrollableElement,
    tag::Tag,
};

use crate::home::card;

// Filled in from the workspace Cargo.toml.
const VERSION: &str = env!("CARGO_PKG_VERSION");
const LICENSE: &str = env!("CARGO_PKG_LICENSE");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const COPYRIGHT_YEAR: &str = "2026";

/// About page: what TSMCL is, how sign-in works, privacy and license.
pub struct About;

impl Render for About {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("about-content")
            .size_full()
            .p_6()
            .gap_4()
            .overflow_y_scrollbar()
            .child(app_card(cx))
            .child(sign_in_info(cx))
            .child(privacy(cx))
            .child(legal_notice(cx))
            .child(copyright(cx))
    }
}

fn app_card(cx: &mut Context<About>) -> impl IntoElement {
    card(cx)
        .p_5()
        .gap_4()
        .child(
            h_flex()
                .gap_4()
                .child(
                    div()
                        .size(px(56.0))
                        .flex_shrink_0()
                        .rounded(cx.theme().radius_lg)
                        .bg(cx.theme().primary)
                        .text_color(cx.theme().primary_foreground)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(Icon::new(IconName::Gamepad2)),
                )
                .child(
                    v_flex()
                        .flex_1()
                        .gap_1()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(div().text_xl().font_semibold().child("TSMCL"))
                                .child(Tag::secondary().small().child(VERSION)),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child("Teishou's Minecraft Launcher：开源的 Minecraft: Java 版第三方启动器"),
                        ),
                ),
        )
        .child(info_row("版本", VERSION, cx))
        .child(info_row("许可证", LICENSE, cx))
        .child(info_row("源代码", REPOSITORY, cx))
        .child(
            h_flex()
                .gap_2()
                .child(
                    Button::new("open-repo")
                        .outline()
                        .icon(IconName::Github)
                        .label("打开 GitHub 仓库")
                        .on_click(|_, _, cx| cx.open_url(REPOSITORY)),
                )
                .child(
                    Button::new("open-license")
                        .outline()
                        .icon(IconName::FileText)
                        .label("查看许可证")
                        .on_click(|_, _, cx| {
                            cx.open_url(&format!("{REPOSITORY}/blob/main/LICENSE.txt"))
                        }),
                ),
        )
}

fn info_row(label: &'static str, value: &'static str, cx: &App) -> impl IntoElement {
    h_flex()
        .gap_4()
        .text_sm()
        .child(div().w(px(80.0)).text_color(cx.theme().muted_foreground).child(label))
        .child(div().flex_1().min_w_0().child(value))
}

fn section(icon: IconName, title: &'static str, cx: &App) -> gpui_kit::Div {
    card(cx).p_5().gap_3().child(
        h_flex()
            .gap_2()
            .child(Icon::new(icon).small().text_color(cx.theme().primary))
            .child(div().font_semibold().child(title)),
    )
}

fn paragraph(text: &'static str, cx: &App) -> impl IntoElement {
    div().text_sm().text_color(cx.theme().muted_foreground).child(text)
}

fn link_button(id: &'static str, label: &'static str, url: &'static str) -> impl IntoElement {
    Button::new(id)
        .ghost()
        .small()
        .icon(IconName::ExternalLink)
        .label(label)
        .on_click(move |_, _, cx| cx.open_url(url))
}

fn legal_notice(cx: &App) -> impl IntoElement {
    section(IconName::Scale, "法律声明", cx)
        .child(paragraph(
            "TSMCL 是独立开发的第三方启动器，不是 Minecraft 官方产品，未经 Mojang Studios 或 Microsoft 批准，也与其无关。",
            cx,
        ))
        .child(paragraph(
            "NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.",
            cx,
        ))
        .child(paragraph(
            "Minecraft 是 Mojang Synergies AB 的商标；Microsoft 和 Xbox 是 Microsoft Corporation 的商标。本软件中提及的其他商标归各自所有者所有，提及它们仅用于说明兼容性。",
            cx,
        ))
        .child(paragraph(
            "TSMCL 不附带、不分发任何 Minecraft 游戏代码或素材，所有游戏文件都在运行时从官方服务器下载。使用 Minecraft 须遵守 Minecraft 最终用户许可协议（EULA）和 Microsoft 服务协议。",
            cx,
        ))
        .child(
            h_flex()
                .gap_1()
                .flex_wrap()
                .child(link_button("eula", "Minecraft EULA", "https://www.minecraft.net/eula"))
                .child(link_button(
                    "usage-guidelines",
                    "Minecraft 使用准则",
                    "https://www.minecraft.net/usage-guidelines",
                ))
                .child(link_button(
                    "ms-services-agreement",
                    "Microsoft 服务协议",
                    "https://www.microsoft.com/servicesagreement",
                )),
        )
}

fn copyright(cx: &App) -> impl IntoElement {
    section(IconName::Copyright, "版权声明", cx)
        .child(div().text_sm().child(format!(
            "Copyright © {COPYRIGHT_YEAR} {AUTHORS} 及 TSMCL 贡献者"
        )))
        .child(paragraph(
            "本程序是自由软件：你可以在自由软件基金会发布的 GNU 通用公共许可证第 3 版（GPL-3.0）的条款下重新发布和/或修改它。",
            cx,
        ))
        .child(paragraph(
            "发布本程序是希望它有用，但不提供任何担保，甚至不包括对适销性或特定用途适用性的默示担保。详情请参阅 GNU 通用公共许可证。",
            cx,
        ))
        .child(div().text_sm().font_semibold().pt_1().child("第三方组件"))
        .child(
            v_flex()
                .gap_1()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child("GPUI、GPUI Component、GPUI Kit：Apache License 2.0")
                .child("Lucide 图标：ISC License，Copyright (c) 2026 Lucide Icons and Contributors")
                .child("其他 Rust 依赖库的许可证见各自的仓库"),
        )
}

fn sign_in_info(cx: &App) -> impl IntoElement {
    section(IconName::ShieldCheck, "登录与正版验证", cx)
        .child(paragraph(
            "TSMCL 只支持 Microsoft 账号登录，不提供离线账号或第三方验证服务。",
            cx,
        ))
        .child(
            v_flex()
                .gap_1()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child("1. 在 Microsoft 官方网页输入设备代码完成登录")
                .child("2. 依次获取 Xbox Live、XSTS 和 Minecraft 令牌")
                .child("3. 检查账号是否拥有 Minecraft: Java 版，未购买的账号无法添加")
                .child("4. 读取玩家档案（名称和 UUID）"),
        )
}

fn privacy(cx: &App) -> impl IntoElement {
    section(IconName::Shield, "隐私", cx)
        .child(paragraph(
            "TSMCL 不会接触你的 Microsoft 密码。登录令牌经过加密后保存在本机，\
             加密密钥存放在系统凭据管理器中。",
            cx,
        ))
        .child(paragraph(
            "令牌只会发送给 Microsoft、Xbox Live 和 Minecraft 的官方服务，以及启动的游戏进程。\
             TSMCL 没有自己的服务器，也不收集任何使用数据。",
            cx,
        ))
}

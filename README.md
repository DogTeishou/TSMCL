# TSMCL — Teishou's Minecraft Launcher

> **TSMCL is an independent, third-party launcher. It is NOT an official Minecraft product
> and is not approved by or associated with Mojang Studios or Microsoft.**

TSMCL is an open-source launcher for Minecraft: Java Edition, written in Rust.
It is in early development (`0.0.5-alpha`); the GUI is still being built.

[中文说明](#中文说明)

## Features

- Sign in with a Microsoft account (the only supported login method)
- Verify that the signed-in account owns Minecraft: Java Edition before the game can be launched
- Download vanilla game versions, libraries and assets from Mojang's official servers
- Install Forge, NeoForge, Fabric and Quilt from their official sources
- Launch the game with configurable Java path, memory, window size and language
- Store several accounts and switch between them

## What TSMCL does not do

- **No offline / local-username accounts.** Earlier development versions contained an
  offline-account helper; it has been removed. A `Player` can only be created through the
  Microsoft sign-in flow below.
- No third-party authentication services (authlib-injector, Yggdrasil servers, etc.).
- No custom or third-party capes, and no selling or trading of capes.
- No bundled Minecraft code or assets. All game files are downloaded from Mojang at runtime.
- No collection of passwords. TSMCL never sees your Microsoft password.

## Authentication flow

TSMCL uses the OAuth 2.0 **device code flow** with its own Azure application ID.
Sign-in happens entirely on Microsoft's official website.

1. **Microsoft account:** TSMCL requests a device code from
   `login.microsoftonline.com/consumers/oauth2/v2.0/devicecode`
   (scope `XboxLive.signin offline_access`). The user opens the official Microsoft
   verification page and enters the code shown by the launcher.
2. **Xbox Live:** the Microsoft access token is exchanged for an Xbox Live token
   (`user.auth.xboxlive.com`).
3. **XSTS:** the Xbox Live token is exchanged for an XSTS token
   (`xsts.auth.xboxlive.com`).
4. **Minecraft:** the XSTS token is exchanged for a Minecraft access token
   (`api.minecraftservices.com/authentication/login_with_xbox`).
5. **Ownership check:** `api.minecraftservices.com/entitlements/mcstore` must contain the
   `product_minecraft` or `game_minecraft` entitlement. Otherwise sign-in fails with
   "no minecraft account".
6. **Profile:** the player's name and UUID are read from
   `api.minecraftservices.com/minecraft/profile`. Accounts without a Java Edition profile are
   rejected.

If any step fails, no account is created and the game cannot be launched. Saved sessions
are renewed with the Microsoft refresh token, and they go through steps 2–6 again,
including the ownership check.

The implementation is in [`auth/src`](auth/src) (`microsoft.rs`, `xbox.rs`, `minecraft.rs`,
`player.rs`).

## Account data and privacy

- Account files are stored locally in `.tsmcl/accounts/<uuid>.toml`.
- The Minecraft access token and the Microsoft refresh token are encrypted with
  ChaCha20-Poly1305 before being written to disk. The encryption key is stored in the
  operating system's credential store (Windows Credential Manager, macOS Keychain, or the
  Linux Secret Service).
- Tokens are only sent to the Microsoft, Xbox Live and Minecraft services listed above,
  and to the game process at launch.
- TSMCL has no servers of its own and does not collect telemetry.

## Building

Requires a recent stable Rust toolchain.

The Azure application (client) ID is read at build time from `TSMCL_CLIENT_ID`. Set it as an
environment variable, or put it in a `.env` file in the repository root:

```
TSMCL_CLIENT_ID=<your-azure-client-id>
```

Then build:

```
cargo build --release -p main
```

## Project layout

| Crate             | Purpose                                                    |
|-------------------|------------------------------------------------------------|
| `auth`            | Microsoft / Xbox / Minecraft authentication, account storage |
| `download`        | Downloading game versions and mod loaders                  |
| `http_downloader` | Concurrent file downloader                                 |
| `launch`          | Building the launch command and starting the game          |
| `model`, `parse`  | Version manifest models and parsing                        |
| `main`            | GUI application                                            |

## License

[GPL-3.0](LICENSE.txt)

Minecraft is a trademark of Mojang Studios / Microsoft. TSMCL is not affiliated with them.

---

## 中文说明

> **TSMCL 是独立的第三方启动器，不是 Minecraft 官方产品，未经 Mojang Studios 或 Microsoft
> 批准，也与其无关。**

TSMCL 是用 Rust 编写的开源 Minecraft: Java 版启动器，目前处于早期开发阶段。

- **登录方式：** 只支持 Microsoft 账号，使用设备码登录，在微软官方网页完成。TSMCL 不会接触你的密码。
- **正版验证：** 账号必须拥有 Minecraft: Java 版才能启动游戏。
- **不支持：** 离线账号、第三方验证服务（authlib-injector 等）、第三方披风。
- **游戏文件：** 全部在运行时从 Mojang 官方服务器下载，不打包任何游戏文件。
- **账号数据：** 令牌经过加密后保存在本地，加密密钥存放在系统凭据管理器中。

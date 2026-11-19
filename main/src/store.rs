//! Shared account state for the GUI: saved accounts, the current account and Microsoft sign-in.
//!
//! The `auth` crate uses reqwest and tokio::fs, so its futures run on the tokio runtime.
//! GPUI tasks only wait for their results and update the UI.

use std::path::PathBuf;

use auth::{error::Error, microsoft::DeviceCodeResponse, player::Player};
use gpui_kit::{ClipboardItem, Context, SharedString};
use tokio::{runtime::Handle, task::AbortHandle};

/// Directory where account files are stored: `<working dir>/.tsmcl/accounts`.
fn accounts_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_default()
        .join(".tsmcl")
        .join("accounts")
}

pub enum LoginState {
    Idle,
    /// Asking Microsoft for a device code.
    Requesting,
    /// Waiting for the user to enter the code on Microsoft's sign-in page.
    Waiting {
        user_code: SharedString,
        verification_uri: SharedString,
    },
    Failed(SharedString),
}

pub struct AccountStore {
    runtime: Handle,
    pub players: Vec<Player>,
    selected: Option<String>,
    pub login: LoginState,
    login_task: Option<AbortHandle>,
    /// Increased for every sign-in attempt, so a cancelled attempt cannot overwrite a newer one.
    login_id: u64,
}

impl AccountStore {
    pub fn new(runtime: Handle, cx: &mut Context<Self>) -> Self {
        let store = Self {
            runtime,
            players: Vec::new(),
            selected: None,
            login: LoginState::Idle,
            login_task: None,
            login_id: 0,
        };
        store.load(cx);
        store
    }

    fn load(&self, cx: &mut Context<Self>) {
        let task = self.runtime.spawn(async { Player::load_all(&accounts_dir()).await });
        cx.spawn(async move |this, cx| {
            if let Ok(Ok(players)) = task.await {
                this.update(cx, |this, cx| {
                    this.players = players;
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    /// The account used to launch the game: the selected one, or the first saved account.
    pub fn current(&self) -> Option<&Player> {
        self.selected
            .as_deref()
            .and_then(|uuid| self.players.iter().find(|p| p.uuid == uuid))
            .or_else(|| self.players.first())
    }

    pub fn is_current(&self, uuid: &str) -> bool {
        self.current().is_some_and(|p| p.uuid == uuid)
    }

    pub fn select(&mut self, uuid: &str, cx: &mut Context<Self>) {
        self.selected = Some(uuid.to_string());
        cx.notify();
    }

    pub fn remove(&mut self, uuid: &str, cx: &mut Context<Self>) {
        self.players.retain(|p| p.uuid != uuid);
        if self.selected.as_deref() == Some(uuid) {
            self.selected = None;
        }
        cx.notify();

        let uuid = uuid.to_string();
        self.runtime.spawn(async move { Player::delete(&accounts_dir(), &uuid).await });
    }

    pub fn is_signing_in(&self) -> bool {
        matches!(self.login, LoginState::Requesting | LoginState::Waiting { .. })
    }

    /// Start the Microsoft device-code sign-in. On success the account is saved and selected.
    pub fn start_login(&mut self, cx: &mut Context<Self>) {
        if self.is_signing_in() {
            return;
        }
        self.login_id += 1;
        let login_id = self.login_id;
        self.login = LoginState::Requesting;
        cx.notify();

        let (code_tx, mut code_rx) = tokio::sync::mpsc::unbounded_channel::<DeviceCodeResponse>();
        let task = self.runtime.spawn(async move {
            let player = Player::new(move |device| {
                let _ = code_tx.send(device);
            })
            .await?;
            let dir = accounts_dir();
            tokio::fs::create_dir_all(&dir).await?;
            player.save(&dir).await?;
            Ok::<Player, Error>(player)
        });
        self.login_task = Some(task.abort_handle());

        cx.spawn(async move |this, cx| {
            if let Some(device) = code_rx.recv().await {
                this.update(cx, |this, cx| {
                    if this.login_id == login_id {
                        this.login = LoginState::Waiting {
                            user_code: device.user_code.into(),
                            verification_uri: device.verification_uri.into(),
                        };
                        cx.notify();
                    }
                })
                .ok();
            }

            let result = task.await;
            this.update(cx, |this, cx| {
                if this.login_id != login_id {
                    return;
                }
                this.login_task = None;
                this.login = match result {
                    Ok(Ok(player)) => {
                        this.selected = Some(player.uuid.clone());
                        match this.players.iter_mut().find(|p| p.uuid == player.uuid) {
                            Some(existing) => *existing = player,
                            None => this.players.push(player),
                        }
                        LoginState::Idle
                    }
                    Ok(Err(e)) => LoginState::Failed(login_error_message(&e).into()),
                    Err(e) if e.is_cancelled() => LoginState::Idle,
                    Err(e) => LoginState::Failed(format!("登录失败：{e}").into()),
                };
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    pub fn cancel_login(&mut self, cx: &mut Context<Self>) {
        if let Some(task) = self.login_task.take() {
            task.abort();
        }
        self.login_id += 1;
        self.login = LoginState::Idle;
        cx.notify();
    }

    /// Copy the device code and open Microsoft's sign-in page in the browser.
    pub fn open_login_page(&self, cx: &mut Context<Self>) {
        if let LoginState::Waiting { user_code, verification_uri } = &self.login {
            cx.write_to_clipboard(ClipboardItem::new_string(user_code.to_string()));
            cx.open_url(verification_uri);
        }
    }
}

fn login_error_message(error: &Error) -> String {
    match error {
        Error::NoMinecraftAccount => "该账号没有购买 Minecraft: Java 版，无法添加。".into(),
        Error::AuthorizationDeclined => "你在 Microsoft 页面拒绝了授权。".into(),
        Error::ExpiredToken => "登录代码已过期，请重新登录。".into(),
        Error::BadVerificationCode => "登录代码无效，请重新登录。".into(),
        Error::MissingClientId => "构建时未设置 TSMCL_CLIENT_ID，无法登录。".into(),
        Error::ReqwestError(_) => format!("网络请求失败：{error}"),
        _ => format!("登录失败：{error}"),
    }
}

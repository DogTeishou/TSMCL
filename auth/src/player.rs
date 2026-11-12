#[derive(Clone)]
pub struct Player {
    pub is_online: bool,
    pub username: String,
    pub uuid: String,
    pub access_token: String,
    pub client_id: Option<String>,
    pub x_uid: Option<String>,
    pub refresh_token: Option<String>,
}

fn generate_offline_uuid(name: &str) -> String {
    let digest = md5::compute(format!("OfflinePlayer:{}", name)).0;
    uuid::Builder::from_md5_bytes(digest).into_uuid().simple().to_string()
}

impl Player {
    pub fn new_offline_player(username: &str) -> Self {
        Self { 
            is_online: false, 
            username: String::from(username), 
            uuid: generate_offline_uuid(username), 
            access_token: String::from("offline"), 
            refresh_token: None,
            client_id: None,
            x_uid: None,
        }
    }
}
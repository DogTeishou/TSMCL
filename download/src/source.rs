pub enum Source {
    Official,
}

impl Source {
    pub fn get_manifests_url<'a>(&self, url: &'a str) -> &'a str {
        match self {
            Self::Official => {
                url
            }
        }
    }

    pub fn get_manifest_url<'a>(&self, url: &'a str) -> &'a str {
        match self {
            Self::Official => {
                url
            }
        }
    }

    pub fn get_library_url<'a>(&self, url: &'a str) -> &'a str {
        match self {
            Source::Official => {
                url
            },
        }
    }

    pub fn get_asset_url<'a>(&self, url: &'a str) -> &'a str {
        match self {
            Source::Official => {
                url
            },
        }
    }
    
    pub fn get_version_url<'a>(&self, url: &'a str) -> &'a str {
        match self {
            Source::Official => {
                url
            },
        }
    }

    pub fn get_forge_url<'a>(&self, url: &'a str) -> &'a str {
        match self {
            Source::Official => {
                url
            }
        }
    }

    pub fn get_fabric_url<'a>(&self, url: &'a str) -> &'a str {
        match self {
            Source::Official => {
                url
            }
        }
    }

    pub fn get_quilt_url<'a>(&self, url: &'a str) -> &'a str {
        match self {
            Source::Official => {
                url
            }
        }
    }
}
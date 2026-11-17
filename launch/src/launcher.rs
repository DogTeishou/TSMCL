use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};

use auth::player::Player;
use model::{Allowed, Argument, ArgumentValue, Library};
use model::Argument::{Conditional, Plain};
use model::version::Version;
use parse::Parse;
use zip::ZipArchive;

use crate::LAUNCHER_VERSION;
use crate::langauge::Lang;
use crate::error::Error;

pub struct Launcher {
    is_iso: bool,
    cmd: Command,
    version_dir: PathBuf,
    player: Player,
    width: Option<usize>,
    height: Option<usize>,
    lang: Lang,
    min_g: usize,
    max_g: usize,
}

impl Launcher {
    pub fn new(java_dir: &str, version_dir: &PathBuf, is_iso: bool, player: &Player) -> Self {
        let cmd = Command::new(java_dir);
        let version_dir = Path::new(version_dir).to_path_buf();
        Self { 
            cmd, 
            is_iso, 
            version_dir, 
            player: player.clone(), 
            width: None, 
            height: None, 
            lang: Lang::EnUs,
            min_g: 2,
            max_g: 4,
        }
    }

    pub fn add_arg(&mut self, arg: &str) -> &mut Self {
        self.cmd.arg(arg);
        self
    }

    pub fn add_args(&mut self, args: &[&str]) -> &mut Self {
        self.cmd.args(args);
        self
    }

    pub fn set_min_g(&mut self, min: usize) -> &mut Self {
        self.min_g = min;
        self
    }

    pub fn set_max_g(&mut self, max: usize) -> &mut Self {
        self.max_g= max;
        self
    }

    pub fn set_lang(&mut self, lang: &Lang) -> &mut Self {
        self.lang = lang.clone();
        self
    }

    pub fn set_width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub fn set_height(&mut self, height: usize) -> &mut Self {
        self.height = Some(height);
        self
    }

    pub fn stop(mut child: Child) -> std::io::Result<ExitStatus> {
        child.kill()?;
        child.wait()
    }

    pub fn change_player(&mut self, player: &Player) -> &mut Self {
        self.player = player.clone();
        self
    }

    pub fn run(&mut self) -> Result<Child, Error> {
        let version = self.get_version_json()?;
        self.extract_natives(&version)?;
        self.init_lang()?;
        let args = self.build_arguments(&version);
        self.cmd.args(args);
        if self.is_iso {
            self.cmd.current_dir(&self.version_dir);
        } else {
            self.cmd.current_dir(&self.version_dir.parent().unwrap().parent().unwrap());
        }
        let child = self.cmd
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn().map_err(|e| Error::IoError(e))?;
        Ok(child)
    }

    fn build_arguments(&self, version: &Version) -> Vec<String> {
        let mut vec= Vec::new();
        let classpath = self.classpath(version);
        if let Some(arguments) = &version.arguments {
            if let Some(default) = &arguments.default_user_jvm {
                vec.extend(self.arguments(default));
            }
            vec.extend(self.arguments(&arguments.jvm));
            vec.push(version.main_class.clone());

            vec.extend(self.arguments(&arguments.game));
        }

        if let Some(arguments) = &version.minecraft_arguments{
            vec.push(format!("-Xms{}G", self.min_g));
            vec.push(format!("-Xmx{}G", self.max_g));
            vec.push("-cp".to_string());
            vec.push(classpath.clone());
            vec.push(version.main_class.clone());
            
            let args = self.old_arguments(arguments);
            let args: Vec<String> = args.iter().map(|v| self.replace_old(v, version)).collect();
            vec.extend(args);
        }

        let mut vec: Vec<String> = vec.iter().map(|v| {
            let mut v = v.clone();
            if v.contains("xuid") && !self.player.is_online {
                v = String::new();
            } else if v.contains("clientid") && !self.player.is_online {
                v = String::new();
            } else if v.contains("clientid") && !self.player.is_online {
                v = String::new();
            }
            let v = self.replace_jvm(&v, &classpath);
            let v = self.replace_old(&v, version);
            self.replace_game(&v, version)
        }).collect();

        set_native_library_path(&mut vec, &self.version_dir.join("natives"));
        vec
    }

    fn classpath(&self, version: &Version) -> String {
        let libraries_dir = self.version_dir.parent().unwrap().parent().unwrap().join("libraries");
        let mut entries = Vec::new();

        for library in &version.libraries {
            match library {
                Library::Normal(normal) => {
                    if library.allowed() {
                        if let Some(artifact) = &normal.downloads.artifact {
                            entries.push(libraries_dir.join(&artifact.path).to_string_lossy().into_owned());
                        }
                        if let Some(classifiers) = &normal.downloads.classifiers {
                            if let Some(native) = &normal.get_natives() {
                                entries.push(libraries_dir.join(&classifiers[native].path).to_string_lossy().into_owned());
                            }
                        }
                    }
                },
                Library::Fabric(fabric) => entries.push(libraries_dir.join(&fabric.get_path()).to_string_lossy().into_owned()),
            }
            
        }

        let version_name = self.version_dir.file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| version.id.clone());
        entries.push(
            self.version_dir
                .join(format!("{}.jar", version_name))
                .to_string_lossy()
                .into_owned(),
        );

        if cfg!(target_os = "windows") {
            entries.join(";")
        } else {
            entries.join(":")
        }
    }
    
    fn init_lang(&self) -> Result<(), Error> {
         let mut option_path = self.version_dir.join("options.txt");
        if !self.is_iso {
            option_path = self.version_dir.parent().unwrap().parent().unwrap().join("options.txt");
        }
        if option_path.exists() {
            return Ok(());
        }
        let mut file = std::fs::File::create(option_path)?;
        
        let initial_options = format!("lang:{}\n", self.lang.get_str());
        file.write_all(initial_options.as_bytes())?;
        file.flush()?;
        
        Ok(())
    }

    fn get_version_json(&self) -> Result<Version, Error> {
        let version_json_name = if let Some(name) = self.version_dir.file_name() {
            name.to_string_lossy().to_string()
        } else {
            String::new()
        };
        let version_json_path = &self.version_dir.join(format!("{}.json", version_json_name));
        let string = std::fs::read_to_string(version_json_path)?;
        Version::parse(&string).map_err(|e| Error::SerdeJsonError(e))
    }

    fn arguments(&self, arguments: &[Argument]) -> Vec<String> {
        let mut args = Vec::new();
        arguments.iter().for_each(|v| 
            match v {
                Plain(text) => args.push(text.clone()),
                Conditional(cond) => {
                    if cond.allowed() {
                        match &cond.value {
                            ArgumentValue::Single(text) => args.push(text.clone()),
                            ArgumentValue::Multiple(items) => args.extend(items.clone()),
                        }
                    }
                }
            }
        );
        
        args
    }

    fn old_arguments(&self, old_args: &str) -> Vec<String> {
        old_args.split(" ").map(|v| v.to_string()).collect()
    }

    fn replace_jvm(&self, string: &str, classpath: &str) -> String {
        let natives = &self.version_dir.join("natives");
        let libraries = self.version_dir.parent().unwrap().parent().unwrap().join("libraries");
        let mut hash_map = HashMap::new();
        hash_map.insert("${natives_directory}".to_string(), natives.to_string_lossy().to_string());
        hash_map.insert("${launcher_name}".to_string(), "Teishou's Minecraft Launcher".to_string());
        hash_map.insert("${launcher_version}".to_string(), LAUNCHER_VERSION.to_string());
        hash_map.insert("${classpath}".to_string(), classpath.to_string());
        hash_map.insert("-Xms2G".to_string(), format!("-Xms{}G", self.min_g));
        hash_map.insert("-Xmx4G".to_string(), format!("-Xmx{}G", self.max_g));
        hash_map.insert("${library_directory}".to_string(), libraries.to_string_lossy().to_string());

        let mut text= string.to_string();

        for (k, v) in &hash_map {
            if string.contains(k) {
                text = string.replace(k, v);
            }
        }

        text
    }

    fn replace_game(&self, string: &str, version: &Version) -> String {
        let game_dir = &self.version_dir.parent().unwrap_or(Path::new("")).parent().unwrap_or(Path::new("")).to_path_buf();
        let game_name = &self.version_dir.to_string_lossy().to_string();
        let mut run_dir = game_dir;
        if self.is_iso {
            run_dir = &self.version_dir;
        }
        let mut hash_map = HashMap::new();
        hash_map.insert("${auth_player_name}".to_string(), self.player.username.to_string());
        hash_map.insert("${version_name}".to_string(), game_name.clone());
        hash_map.insert("${game_directory}".to_string(), run_dir.clone().to_string_lossy().to_string());
        hash_map.insert("${assets_root}".to_string(), game_dir.join("assets").to_string_lossy().to_string());
        hash_map.insert("${assets_index_name}".to_string(), version.assets.clone());
        hash_map.insert("${auth_uuid}".to_string(), self.player.uuid.clone());
        hash_map.insert("${auth_access_token}".to_string(), self.player.access_token.clone());
        hash_map.insert("${version_type}".to_string(), format!("TSMCL: {}", LAUNCHER_VERSION));
        hash_map.insert(
            "${user_type}".to_string(),
            if self.player.is_online { "msa" } else { "legacy" }.to_string(),
        );
        
        if let Some(client_id) = self.player.client_id.clone() && self.player.is_online {
            hash_map.insert("${clientid}".to_string(), client_id);
        }
        if let Some(x_uid) = self.player.x_uid.clone() && self.player.is_online {
            hash_map.insert("${auth_xuid}".to_string(), x_uid);
        }
        if let Some(width) = self.width {
            hash_map.insert("${resolution_width}".to_string(), width.to_string());
        }
        if let Some(height) = self.height {
            hash_map.insert("${resolution_height}".to_string(), height.to_string());
        }

        let mut text= string.to_string();

        for (k, v) in &hash_map {
            if !string.contains("$") {
                continue;
            }
            if string.eq(k) {
                text = string.replace(k, v);
            }
        }

        text
    }

    fn replace_old(&self, string: &String, version: &Version) -> String {
        let mut text = string.clone();
        let mut hash_map = HashMap::new();
        let game_dir = self.version_dir.parent()
            .unwrap_or(Path::new(""))
            .parent()
            .unwrap_or(Path::new(""));
        let mut run_dir = game_dir.to_string_lossy().to_string();
        if self.is_iso {
            run_dir = self.version_dir.to_string_lossy().to_string();
        }
        hash_map.insert("${auth_player_name}", self.player.username.clone());
        hash_map.insert("${version_name}", version.id.clone());
        hash_map.insert("${game_directory}", run_dir);
        hash_map.insert("${assets_root}", game_dir.join("assets").to_string_lossy().to_string());
        hash_map.insert("${assets_index_name}", version.assets.clone());
        hash_map.insert("${auth_uuid}", self.player.uuid.clone());
        hash_map.insert("${auth_access_token}", self.player.access_token.clone());
        if self.player.is_online {
            hash_map.insert("${user_type}", "msa".to_string());
        } else {
            hash_map.insert("${user_type}", "legacy".to_string());
        }
        hash_map.insert("${version_type}", format!("TSMCL: {}", LAUNCHER_VERSION));

        for (key, value) in hash_map {
            if string.contains(key) {
                text = string.replace(key, &value);
            }
        }

        text
    }

    fn extract_natives(&self, version: &Version) -> Result<(), Error> {
        let libraries_dir = self.version_dir.parent().unwrap().parent().unwrap().join("libraries");
        let natives_dir = self.version_dir.join("natives");
        if !natives_dir.exists() {
            std::fs::create_dir_all(&natives_dir)?;
        }

    
        for lib in &version.libraries {
            let lib = match lib {
                Library::Normal(normal_library) => normal_library,
                Library::Fabric(_) => continue,
            };
            if !lib.allowed() {
                continue;
            }
            for native in native_artifacts(lib) {
                extract_jar(&libraries_dir.join(&native.path), &natives_dir)?;
            }
        }
        Ok(())
    }
}

fn set_native_library_path(args: &mut Vec<String>, natives_dir: &Path) {
    let native_path = format!("-Djava.library.path={}", natives_dir.to_string_lossy());
    let mut found = false;
    for arg in args.iter_mut() {
        if arg.starts_with("-Djava.library.path=") {
            *arg = native_path.clone();
            found = true;
        }
    }
    if !found {
        args.insert(0, native_path);
    }
}

fn native_artifacts(library: &model::NormalLibrary) -> Vec<&model::DownloadItem> {
    let mut artifacts = Vec::new();
    if library.name.contains("natives")
        && let Some(artifact) = &library.downloads.artifact
    {
        artifacts.push(artifact);
    }
    if let Some(native) = library.get_natives()
        && let Some(artifact) = library
            .downloads
            .classifiers
            .as_ref()
            .and_then(|classifiers| classifiers.get(&native))
    {
        artifacts.push(artifact);
    }
    artifacts
}

#[cfg(test)]
mod tests {
    use super::set_native_library_path;
    use std::path::Path;

    #[test]
    fn adds_native_path_for_legacy_versions() {
        let mut args = vec!["-Xmx4G".to_string(), "net.minecraft.launchwrapper.Launch".to_string()];

        set_native_library_path(&mut args, Path::new("natives"));

        assert_eq!(args[0], "-Djava.library.path=natives");
    }

    #[test]
    fn replaces_existing_native_path() {
        let mut args = vec![
            "-Djava.library.path=old".to_string(),
            "net.minecraft.client.main.Main".to_string(),
        ];

        set_native_library_path(&mut args, Path::new("natives"));

        assert_eq!(args[0], "-Djava.library.path=natives");
        assert_eq!(args.len(), 2);
    }
}

fn extract_jar(jar: &Path, dir: &Path) -> Result<(), Error> {
    let mut archive = ZipArchive::new(std::fs::File::open(jar)?)?;
    std::fs::create_dir_all(dir)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        if entry.name()?.starts_with("META-INF/") { continue; }

        let Some(rel) = entry.enclosed_name() else {
            continue;
        };
        let out = dir.join(rel);

        if entry.is_dir() {
            std::fs::create_dir_all(&out)?;
            continue;
        }

        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = std::fs::File::create(&out)?;
        std::io::copy(&mut entry, &mut file)?;
        
        #[cfg(unix)]
        if let Some(mode) = entry.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&out, fs::Permissions::from_mode(mode))?;
        }
    }
    Ok(())
}

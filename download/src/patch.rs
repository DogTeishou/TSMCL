use std::{collections::HashMap, ffi::OsStr, io::Read, path::{Path, PathBuf}, process::{Command, Stdio}};

use http_downloader::{Downloader, verify};
use model::{Allowed, Library::{self, Fabric, Normal}, forge::ForgeVersion, forge_manifest::{ForgeInstallProfile, Patched}, processor::Processor};
use parse::Parse;
use zip::ZipArchive;

use crate::{BuildTasks, Patch, Source, error::Error};

impl Patch for ForgeInstallProfile {
    type Version = ForgeVersion;

    async fn patch(&self, downloader: &Downloader, source: &Source, jar_dir: &str ,tmp_dir: &Path, ver_dir: &Path) -> Result<Self::Version, Error> {
        let game_dir = if let Some(versions) = ver_dir.parent() && let Some(game) = versions.parent() {
            game
        } else {
            return Err(Error::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "path not found")));
        };

        let libraries_dir = game_dir.join("libraries");
        if !libraries_dir.exists() {
            return Err(Error::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "path not found")));
        }
        
        self.libraries.build_tasks(downloader, source, game_dir).await?;
        downloader.download().await?;
 
        if let Some(processors) = &self.processors {
            for processor in processors {
                if !processor.allowed() {
                    continue;
                }
                do_processor(jar_dir, processor, &self.libraries, ver_dir, &libraries_dir, &tmp_dir, self.data.as_ref()).await?;
            }
        }
        let ver_path= tmp_dir.join("version.json");
        let ver = ForgeVersion::parse(&std::fs::read_to_string(ver_path)?)?;
        Ok(ver)
    }
}

async fn do_processor(
    jar_dir: &str,
    processor: &Processor,
    libraries: &[Library],
    version_dir: &Path,
    libraries_dir: &Path, 
    tmp_dir: &Path,
    data: Option<&HashMap<String, Patched>>) -> Result<(), Error> 
{
    let mut main_jar = String::new();
    for lib in libraries {
        match lib {
            Normal(normal_library) => if normal_library.name.eq(&processor.jar) && let Some(artifact) = &normal_library.downloads.artifact{
                main_jar = artifact.path.clone();
            },
            Fabric(_) => continue,
        }
    };
    let data = if let Some(data) = data {
        data
    } else {
        return Err(Error::PatchFailedError);
    };
    let version_dir_name = version_dir.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string();
    let version_jar = version_dir.join(format!("{}.jar", version_dir_name));
    let main_jar_path = &libraries_dir.join(main_jar);
    let main_class = get_jar_main_class(main_jar_path)?;
    let classpath = gen_classpath(&processor.classpath, libraries, libraries_dir, &main_jar_path.to_string_lossy().to_string());
    let mut sha1 = String::new();
    let mut client_patched= PathBuf::new();
    let args: Vec<String> = processor.args.iter().map(|v| 
        if v.contains("{MINECRAFT_JAR}") {
            version_dir.join(version_jar.clone()).to_string_lossy().to_string()
        } else if v.contains("{PATCHED}") {
            let path = data["PATCHED"].client.clone();
            let splits: Vec<&str> = path.split(":").collect();
            let client_jar;
            let path;
            if !splits[0].contains("neoforged") {
                client_jar = format!("{}-{}-{}.jar", splits[1], splits[2], splits[3]).replace("]", "");
                path = format!("{};{};{}{}", splits[0].replace(".", &get_file_system_split()), splits[1], splits[2], get_file_system_split())
                    .replace("[", "")
                    .replace(";", &get_file_system_split());
            } else {
                client_jar = format!("minecraft-client-patched-{}.jar", splits[2]).replace("]", "");
                path = format!("{};{};{}{}", splits[0].replace(".", &get_file_system_split()), splits[1], splits[2], get_file_system_split())
                    .replace("[", "")
                    .replace("]", "")
                    .replace(";", &get_file_system_split());
            }
            client_patched = libraries_dir.join(format!("{}{}", path, client_jar));
            if client_patched.exists() {
                sha1 = data["PATCHED_SHA"].client.clone().replace("'", "");
            }
            client_patched.to_string_lossy().to_string()
        } else if v.contains("{BINPATCH}") {
            tmp_dir.join("data").join("client.lzma").to_string_lossy().to_string()
        } else {
            v.clone()
        }).collect();
    if verify(&sha1, &client_patched).await {
        return Ok(());
    }
    let mut child = Command::new(jar_dir)
        .current_dir(main_jar_path.parent().unwrap_or(Path::new("")))
        .arg("-cp")
        .arg(classpath)
        .arg(main_class)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;
    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::PatchFailedError)
    }
}

fn get_file_system_split() -> String {
    if cfg!(target_os = "windows") {
        String::from("\\")
    } else {
        String::from("/")
    }
}

fn gen_classpath(classes: &[String], libraries: &[Library], libraries_dir: &Path, main_jar: &str) -> String {
    let mut vec = Vec::new();
    for class in classes {
        for lib in libraries {
            match lib {
                Normal(normal) => {
                    if normal.name.eq(class) && let Some(artifact) = &normal.downloads.artifact {
                        let path = libraries_dir.join(artifact.path.clone()).to_string_lossy().to_string();
                        #[cfg(windows)]
                        let path = path.replace("/", "\\");
                        vec.push(path);
                    }
                },
                Fabric(_) => continue,
            }
        }
    }
    #[cfg(windows)]
    let main_jar= main_jar.replace("/", "\\");
    vec.push(main_jar);
    if cfg!(target_os = "windows") { vec.join(";") } else { vec.join(":") }
}

fn get_jar_main_class(path: &Path) -> Result<String, Error> {
    let mut zip = ZipArchive::new(std::fs::File::open(path)?)?;
    let mut main_class = String::new();
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        if !entry.name()?.contains("META_INF/") && !entry.name()?.contains("MANIFEST.MF") {
            continue;
        }
        let mut string = String::new();
        entry.read_to_string(&mut string)?;
        if let Some(class) = parse_main_class(&string) {
            main_class = class.to_string();
        }
    }
    Ok(main_class)
}

fn parse_main_class(manifest: &str) -> Option<&str> {
    manifest
        .lines()
        .find_map(|line| line.strip_prefix("Main-Class:").map(str::trim))
        .filter(|class| !class.is_empty())
}
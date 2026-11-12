use auth::player::Player;

use crate::{langauge::Lang, launcher::Launcher};

#[test]
fn test_26_3_neo() -> Result<(), Box<dyn std::error::Error>> {
    let version_dir = &std::env::current_dir().unwrap().parent().unwrap().join(".minecraft")
        .join("versions")
        .join("26.3-NeoForge_26.3.0.102");
    let mut child = Launcher::new(
        "C:\\Program Files\\Zulu\\zulu-25\\bin\\java.exe", 
        version_dir, 
        true, 
        &Player::new_offline_player("test1"))
        .set_lang(&Lang::ZhCn)
        .run()?;
    let code = child.wait()?;
    if !code.success() {
        return Err(format!("Minecraft exited unsuccessfully: {code}").into());
    }
    println!("Minecraft Run Succeeded");
    Ok(())
}

#[test]
fn test_26_3_forge() -> Result<(), Box<dyn std::error::Error>> {
    let version_dir = &std::env::current_dir().unwrap().parent().unwrap().join(".minecraft")
        .join("versions")
        .join("26.3-Forge_66.1.5");
    let mut child = Launcher::new(
        "C:\\Program Files\\Zulu\\zulu-25\\bin\\java.exe", 
        version_dir, 
        true, 
        &Player::new_offline_player("test1"))
        .set_lang(&Lang::ZhCn)
        .run()?;
    let code = child.wait()?;
    if code.success() {
        println!("Minecraft Run Successed")
    } else {
        println!("Minecraft Run Failed")
    }
    Ok(())
}

#[test]
fn test_26_3_fab() -> Result<(), Box<dyn std::error::Error>> {
    let version_dir = &std::env::current_dir().unwrap().parent().unwrap().join(".minecraft")
        .join("versions")
        .join("26.3-Fabric_0.20.1");
    let mut child = Launcher::new(
        "C:\\Program Files\\Zulu\\zulu-25\\bin\\java.exe", 
        version_dir, 
        true, 
        &Player::new_offline_player("test1"))
        .set_lang(&Lang::ZhCn)
        .run()?;
    let code = child.wait()?;
    if code.success() {
        println!("Minecraft Run Successed")
    } else {
        println!("Minecraft Run Failed")
    }
    Ok(())
}

#[test]
fn test_26_3_qua() -> Result<(), Box<dyn std::error::Error>> {
    let version_dir = &std::env::current_dir().unwrap().parent().unwrap().join(".minecraft")
        .join("versions")
        .join("26.3-Quilt_0.30.2");
    let mut child = Launcher::new(
        "C:\\Program Files\\Zulu\\zulu-25\\bin\\java.exe", 
        version_dir, 
        true, 
        &Player::new_offline_player("test1"))
        .set_lang(&Lang::ZhCn)
        .run()?;
    let code = child.wait()?;
    if code.success() {
        println!("Minecraft Run Successed")
    } else {
        println!("Minecraft Run Failed")
    }
    Ok(())
}
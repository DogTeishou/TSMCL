use auth::player::Player;

use crate::{langauge::Lang, launcher::Launcher};

#[test]
fn test_26_3_neo() -> Result<(), Box<dyn std::error::Error>> {
    let version_dir = &std::env::current_dir().unwrap().parent().unwrap().join(".minecraft")
        .join("versions")
        .join("neoforge-26.3-26.3.0.110");
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
        .join("forge-26.3-66.1.8");
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
fn test_1_12_2_forge() -> Result<(), Box<dyn std::error::Error>> {
    let version_dir = &std::env::current_dir().unwrap().parent().unwrap().join(".minecraft")
        .join("versions")
        .join("forge-1.12.2-14.23.5.2859");
    let mut child = Launcher::new(
        "C:\\Program Files\\Zulu\\zulu-8\\bin\\java.exe",
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
fn test_26_3_fab() -> Result<(), Box<dyn std::error::Error>> {
    let version_dir = &std::env::current_dir().unwrap().parent().unwrap().join(".minecraft")
        .join("versions")
        .join("fabric-26.3-0.20.1");
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
fn test_26_3_qui() -> Result<(), Box<dyn std::error::Error>> {
    let version_dir = &std::env::current_dir().unwrap().parent().unwrap().join(".minecraft")
        .join("versions")
        .join("quilt-26.3-0.30.2");
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
    println!("Minecraft Run Successed");
    Ok(())
}
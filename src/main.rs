use clap::{Command, Arg, ArgMatches};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use rust_i18n::t;
use sys_locale::get_locale;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;
#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{SendMessageTimeoutW, HWND_BROADCAST, WM_SETTINGCHANGE, SMTO_ABORTIFHUNG};
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::LPARAM;

// 设置 i18n
rust_i18n::i18n!("locales");

#[derive(Serialize, Deserialize)]
struct JdkConfig {
    jdks: HashMap<String, String>,
}

fn main() {
    // 设置语言
    let locale = get_locale().unwrap_or_else(|| String::from("en"));
    rust_i18n::set_locale(&locale);

    let matches = build_cli().get_matches();

    if let Err(e) = run(matches) {
        eprintln!("{}", t!("error", message = e.to_string()));
        std::process::exit(1);
    }
}

fn build_cli() -> Command {
    Command::new("jdkman")
        .about(&t!("app_description"))
        .subcommand(
            Command::new("add")
                .about(&t!("add_description"))
                .arg(Arg::new("name").help(&t!("add_name_help")).required(true))
                .arg(Arg::new("path").help(&t!("add_path_help")).required(true))
        )
        .subcommand(
            Command::new("remove")
                .about(&t!("remove_description"))
                .arg(Arg::new("name").help(&t!("remove_name_help")).required(true))
        )
        .subcommand(Command::new("list").about(&t!("list_description")))
        .subcommand(
            Command::new("activate")
                .about(&t!("activate_description"))
                .arg(Arg::new("name").help(&t!("activate_name_help")).required(true))
        )
}

fn run(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::home_dir()
        .ok_or("Unable to get user home directory")?
        .join(".jdkman");
    fs::create_dir_all(&config_dir)?;
    let config_file = config_dir.join("config.json");

    let mut config = if config_file.exists() {
        let content = fs::read_to_string(&config_file)?;
        serde_json::from_str(&content).unwrap_or(JdkConfig {
            jdks: HashMap::new(),
        })
    } else {
        JdkConfig {
            jdks: HashMap::new(),
        }
    };

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            let path = sub_matches.get_one::<String>("path").unwrap();
            config.jdks.insert(name.clone(), path.clone());
            println!("{}", t!("add_jdk", name = name, path = path));
        }
        Some(("remove", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            if config.jdks.remove(name).is_some() {
                println!("{}", t!("remove_jdk", name = name));
            } else {
                println!("{}", t!("jdk_not_found", name = name));
            }
        }
        Some(("list", _)) => {
            if config.jdks.is_empty() {
                println!("{}", t!("no_jdks"));
            } else {
                println!("{}", t!("added_jdks"));
                for (name, path) in &config.jdks {
                    println!("  {} -> {}", name, path);
                }
            }
        }
        Some(("activate", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            if let Some(path) = config.jdks.get(name) {
                set_java_home(path)?;
                println!("{}", t!("activate_jdk", name = name, path = path));
            } else {
                println!("{}", t!("jdk_not_found", name = name));
            }
        }
        _ => {
            println!("{}", t!("use_help"));
        }
    }

    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(config_file, config_json)?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn set_java_home(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let environment = hkcu.open_subkey_with_flags(
        r"Environment",
        KEY_READ | KEY_WRITE,
    )?;

    environment.set_value("JAVA_HOME", &path)?;

    // 构造绝对路径
    let java_home_bin = Path::new(path).join("bin").to_string_lossy().into_owned();

    // 更新 PATH 环境变量
    let current_path: String = environment.get_value("PATH")?;
    let new_path = if !current_path.to_lowercase().contains(&java_home_bin.to_lowercase()) {
        format!("{};{}", java_home_bin, current_path)
    } else {
        // 如果已存在，替换旧路径
        let parts: Vec<&str> = current_path.split(';').collect();
        let new_parts: Vec<String> = parts
            .into_iter()
            .map(|part| {
                if part.to_lowercase().contains("java") && part.to_lowercase().ends_with("bin") {
                    java_home_bin.clone()
                } else {
                    part.to_string()
                }
            })
            .collect();
        new_parts.join(";")
    };
    environment.set_value("PATH", &new_path)?;

    // 通知 Windows 环境变量已更改
    let env_str: Vec<u16> = OsStr::new("Environment").encode_wide().chain(std::iter::once(0)).collect();
    unsafe {
        SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            env_str.as_ptr() as LPARAM,
            SMTO_ABORTIFHUNG,
            5000,
            std::ptr::null_mut(),
        );
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn set_java_home(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Unable to get home directory")?;
    let shell_rc = if cfg!(target_os = "macos") {
        home_dir.join(".zshrc")
    } else {
        home_dir.join(".bashrc")
    };

    let mut content = fs::read_to_string(&shell_rc)?;
    
    // Remove old JAVA_HOME and PATH entries
    content = content.lines()
        .filter(|line| !line.starts_with("export JAVA_HOME=") && !line.contains("$JAVA_HOME/bin"))
        .collect::<Vec<&str>>()
        .join("\n");

    // Add new JAVA_HOME and PATH entries
    content.push_str(&format!("\nexport JAVA_HOME={}", path));
    content.push_str("\nexport PATH=$JAVA_HOME/bin:$PATH\n");

    fs::write(&shell_rc, content)?;

    println!("Please run 'source {}' or restart your terminal to apply changes.", shell_rc.display());

    Ok(())
}
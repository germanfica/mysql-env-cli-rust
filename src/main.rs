use clap::{Arg, ArgAction, Command};
use std::collections::HashSet;
use winreg::enums::*;
use winreg::RegKey;

const MYSQL_VERSIONS: &[&str] = &[
    "8.0.0", "8.0.1", "8.0.2", "8.0.11", "8.0.12", "8.0.13", "8.0.14", "8.0.15", "8.0.16",
    "8.0.17", "8.0.18", "8.0.19", "8.0.20", "8.0.21", "8.0.22", "8.0.23", "8.0.24", "8.0.25",
    "8.0.26", "8.0.27", "8.0.28", "8.0.29", "8.0.30", "8.0.31", "8.0.32", "8.0.33", "8.0.34",
    "8.0.35", "8.0.36", "8.0.37", "8.0.39", "8.1.0", "8.1.1", "8.2.0", "8.4.2", "9.0.1",
];

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches =
        Command::new("mysql_env")
            .about("MySQL environment manager")
            .version(APP_VERSION)
            .subcommand_required(true)
            .arg_required_else_help(true)
            // Install subcommand
            .subcommand(
                Command::new("install")
                    .about("Install a specific MySQL version")
                    .arg(Arg::new("manual").short('m').long("manual").help(
                        "Manually enter the MySQL version to install, or specify it directly",
                    ))
                    .arg(
                        Arg::new("list")
                            .short('l')
                            .long("list")
                            .help("Select a version from a predefined list")
                            .action(ArgAction::SetTrue),
                    ),
            )
            // Uninstall subcommand
            .subcommand(
                Command::new("uninstall").about("Uninstall MySQL and remove environment variables"),
            )
            .get_matches();

    match matches.subcommand() {
        Some(("install", install_matches)) => {
            // if install_matches.get_flag("manual") {
            //     if let Some(version) = enter_version_manually() {
            //         install_version(version);
            //     }
            // }
            if let Some(version) = install_matches.get_one::<String>("manual") {
                // Aquí se pasa directamente la versión introducida con -m
                install_version(version.to_string());
            } else if install_matches.get_flag("list") {
                if let Some(version) = select_from_list() {
                    install_version(version);
                }
            } else {
                println!("Please specify either --manual (-m) with a version or --list (-l) to install a version.");
            }
        }
        Some(("uninstall", _)) => {
            desinstalar();
        }
        _ => unreachable!(),
    }
}

fn install_version(version: String) {
    let lib_dir = format!(r"C:\mysql-{}-winx64\lib", version);
    let bin_dir = format!(r"C:\mysql-{}-winx64\bin", version);

    if let Err(e) = set_environment_variable("MYSQLCLIENT_LIB_DIR", &lib_dir) {
        eprintln!("Error setting MYSQLCLIENT_LIB_DIR: {}", e);
    }

    if let Err(e) = set_environment_variable("MYSQLCLIENT_VERSION", &version) {
        eprintln!("Error setting MYSQLCLIENT_VERSION: {}", e);
    }

    if let Err(e) = update_path_variable(&bin_dir) {
        eprintln!("Error updating PATH: {}", e);
    }

    println!(
        "✅ MySQL {} installed and environment variables updated successfully.",
        version
    );
}

fn select_from_list() -> Option<String> {
    let mut major_versions = HashSet::new();
    for version in MYSQL_VERSIONS {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() >= 2 {
            let major_version = format!("{}.{}", parts[0], parts[1]);
            major_versions.insert(major_version);
        }
    }
    let mut major_versions_vec: Vec<String> = major_versions.into_iter().collect();
    major_versions_vec.sort();

    println!("Select the major version of MySQL from the following list:");
    for (i, major_version) in major_versions_vec.iter().enumerate() {
        println!("{}. {}", i + 1, major_version);
    }
    println!("Enter the corresponding number:");

    let mut selected_major = String::new();
    std::io::stdin()
        .read_line(&mut selected_major)
        .expect("Error reading selection");

    let major_index = match selected_major.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= major_versions_vec.len() => index - 1,
        _ => {
            println!("Invalid selection.");
            return None;
        }
    };
    let selected_major_version = &major_versions_vec[major_index];

    let minor_versions: Vec<&str> = MYSQL_VERSIONS
        .iter()
        .filter(|version| version.starts_with(selected_major_version))
        .cloned()
        .collect();

    println!("Select the specific version of MySQL:");
    for (i, version) in minor_versions.iter().enumerate() {
        println!("{}. {}", i + 1, version);
    }
    println!("Enter the corresponding number:");

    let mut selected_minor = String::new();
    std::io::stdin()
        .read_line(&mut selected_minor)
        .expect("Error reading selection");

    let minor_index = match selected_minor.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= minor_versions.len() => index - 1,
        _ => {
            println!("Invalid selection.");
            return None;
        }
    };
    Some(minor_versions[minor_index].to_string())
}

// fn enter_version_manually() -> Option<String> {
//     println!("Enter the version of MySQL to configure (e.g., 8.0.37):");

//     let mut version = String::new();
//     std::io::stdin()
//         .read_line(&mut version)
//         .expect("Error reading version");

//     let version = version.trim().to_string();

//     let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
//     if re.is_match(&version) {
//         Some(version)
//     } else {
//         eprintln!("Invalid version format. Expected format: x.x.x");
//         None
//     }
// }

fn desinstalar() {
    println!("Uninstalling MySQL and removing environment variables...");

    if let Err(e) = delete_environment_variable("MYSQLCLIENT_LIB_DIR") {
        eprintln!("Error deleting MYSQLCLIENT_LIB_DIR: {}", e);
    }

    if let Err(e) = delete_environment_variable("MYSQLCLIENT_VERSION") {
        eprintln!("Error deleting MYSQLCLIENT_VERSION: {}", e);
    }

    if let Err(e) = clean_path_variable() {
        eprintln!("Error cleaning PATH: {}", e);
    }

    println!("✅ MySQL environment variables removed.");
}

fn set_environment_variable(var: &str, value: &str) -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_WRITE)?;
    env.set_value(var, &value)
}

fn delete_environment_variable(var: &str) -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_WRITE)?;
    env.delete_value(var)
}

fn update_path_variable(new_bin_dir: &str) -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    let mut path: String = env.get_value("Path")?;

    path = path
        .split(';')
        .filter(|dir| !dir.contains("mysql-"))
        .collect::<Vec<_>>()
        .join(";");

    path = format!("{};{}", path, new_bin_dir);

    env.set_value("Path", &path)
}

fn clean_path_variable() -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    let mut path: String = env.get_value("Path")?;

    path = path
        .split(';')
        .filter(|dir| !dir.contains("mysql-"))
        .collect::<Vec<_>>()
        .join(";");

    env.set_value("Path", &path)
}

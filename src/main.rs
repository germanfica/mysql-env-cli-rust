use std::io;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    // Verificar si el sistema operativo es Windows
    if !cfg!(target_os = "windows") {
        eprintln!("Este CLI solo es compatible con Windows.");
        std::process::exit(1);
    }

    println!("Ingrese la versión de MySQL que desea configurar (por ejemplo, 8.0.37):");

    let mut version = String::new();
    io::stdin()
        .read_line(&mut version)
        .expect("Error al leer la versión");

    let version = version.trim();

    // Definir los nuevos valores basados en la versión ingresada
    let lib_dir = format!(r"C:\mysql-{}-winx64\lib", version);
    let bin_dir = format!(r"C:\mysql-{}-winx64\bin", version);

    // Actualizar las variables de entorno en Windows
    if let Err(e) = set_environment_variable("MYSQLCLIENT_LIB_DIR", &lib_dir) {
        eprintln!("Error al configurar MYSQLCLIENT_LIB_DIR: {}", e);
    }

    if let Err(e) = set_environment_variable("MYSQLCLIENT_VERSION", version) {
        eprintln!("Error al configurar MYSQLCLIENT_VERSION: {}", e);
    }

    if let Err(e) = update_path_variable(&bin_dir) {
        eprintln!("Error al actualizar el PATH: {}", e);
    }

    println!("Variables de entorno actualizadas correctamente.");
}

fn set_environment_variable(var: &str, value: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_WRITE)?;

    env.set_value(var, &value)
}

fn update_path_variable(new_bin_dir: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    let mut path: String = env.get_value("Path")?;

    // Eliminar todas las versiones anteriores de MySQL en el PATH
    path = path
        .split(';')
        .filter(|dir| !dir.contains("mysql-"))
        .collect::<Vec<_>>()
        .join(";");

    // Agregar la nueva versión de MySQL al PATH
    path = format!("{};{}", path, new_bin_dir);

    env.set_value("Path", &path)
}

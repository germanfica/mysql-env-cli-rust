use regex::Regex;
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

    loop {
        println!("Seleccione una opción:");
        println!("1. Instalar");
        println!("2. Desinstalar");
        println!("3. Salir");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Error al leer la opción");

        match option.trim() {
            "1" => instalar(),
            "2" => desinstalar(),
            "3" => {
                println!("Gracias por usar este CLI. ¡Espero que vuelvas pronto!");
                break;
            }
            _ => println!("Opción no válida, por favor intente de nuevo."),
        }
    }
}

fn instalar() {
    println!("Ingrese la versión de MySQL que desea configurar (por ejemplo, 8.0.37):");

    let mut version = String::new();
    io::stdin()
        .read_line(&mut version)
        .expect("Error al leer la versión");

    let version = version.trim();

    // Validar el formato de la versión
    let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    if !re.is_match(version) {
        eprintln!("Formato de versión no válido. Debe ser en el formato x.x.x");
        return;
    }

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

fn desinstalar() {
    println!("Desinstalando...");

    // Borrar las variables de entorno
    if let Err(e) = delete_environment_variable("MYSQLCLIENT_LIB_DIR") {
        eprintln!("Error al eliminar MYSQLCLIENT_LIB_DIR: {}", e);
    }

    if let Err(e) = delete_environment_variable("MYSQLCLIENT_VERSION") {
        eprintln!("Error al eliminar MYSQLCLIENT_VERSION: {}", e);
    }

    if let Err(e) = clean_path_variable() {
        eprintln!("Error al limpiar el PATH: {}", e);
    }

    println!("Variables de entorno eliminadas correctamente.");
}

fn set_environment_variable(var: &str, value: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_WRITE)?;

    env.set_value(var, &value)
}

fn delete_environment_variable(var: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_WRITE)?;

    env.delete_value(var)
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

fn clean_path_variable() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

    let mut path: String = env.get_value("Path")?;

    // Eliminar todas las versiones de MySQL en el PATH
    path = path
        .split(';')
        .filter(|dir| !dir.contains("mysql-"))
        .collect::<Vec<_>>()
        .join(";");

    env.set_value("Path", &path)
}

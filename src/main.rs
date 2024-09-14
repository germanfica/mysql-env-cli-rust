use regex::Regex;
use std::io;
use winreg::enums::*;
use winreg::RegKey;

const MYSQL_VERSIONS: &[&str] = &[
    "8.0.0", "8.0.1", "8.0.2", "8.0.11", "8.0.12", "8.0.13", "8.0.14", "8.0.15", "8.0.16",
    "8.0.17", "8.0.18", "8.0.19", "8.0.20", "8.0.21", "8.0.22", "8.0.23", "8.0.24", "8.0.25",
    "8.0.26", "8.0.27", "8.0.28", "8.0.29", "8.0.30", "8.0.31", "8.0.32", "8.0.33", "8.0.34",
    "8.0.35", "8.0.36", "8.0.37", "8.0.39", "8.1.0", "8.1.1", "8.2.0", "8.4.2", "9.0.1",
];

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
        println!("q. Salir");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Error al leer la opción");

        match option.trim() {
            "1" => instalar(),
            "2" => desinstalar(),
            "q" => {
                println!("Gracias por usar este CLI. ¡Espero que vuelvas pronto!");
                break;
            }
            _ => println!("Opción no válida, por favor intente de nuevo."),
        }
    }
}

fn select_from_list() -> Option<String> {
    println!("Seleccione la versión de MySQL de la siguiente lista:");
    for (i, version) in MYSQL_VERSIONS.iter().enumerate() {
        println!("{}. {}", i + 1, version);
    }

    let mut selected = String::new();
    io::stdin()
        .read_line(&mut selected)
        .expect("Error al leer la selección");

    match selected.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= MYSQL_VERSIONS.len() => {
            Some(MYSQL_VERSIONS[index - 1].to_string()) // Convertimos &str a String
        }
        _ => {
            println!("Selección no válida.");
            None
        }
    }
}

fn enter_version_manually() -> Option<String> {
    println!("Ingrese la versión de MySQL que desea configurar (por ejemplo, 8.0.37):");

    let mut version = String::new();
    io::stdin()
        .read_line(&mut version)
        .expect("Error al leer la versión");

    let version = version.trim().to_string();

    // Validar el formato de la versión
    let re = Regex::new(r"^\d+.\d+.\d+$").unwrap();
    if re.is_match(&version) {
        Some(version)
    } else {
        eprintln!("Formato de versión no válido. Debe ser en el formato x.x.x");
        None
    }
}

fn instalar() {
    loop {
        println!("¿Desea seleccionar una versión de la lista o ingresar manualmente?");
        println!("1. Seleccionar de la lista");
        println!("2. Ingresar manualmente");
        println!("q. Regresar al menú principal");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error al leer la opción");

        let choice = choice.trim();

        if choice.eq_ignore_ascii_case("q") {
            break;
        }

        let version = match choice {
            "1" => select_from_list(), // el primer valor modifica el retorno // enter_version_manually(),
            "2" => enter_version_manually(),
            _ => {
                println!("Opción no válida, por favor intente de nuevo.");
                continue;
            }
        };

        if let Some(version) = version {
            let lib_dir = format!(r"C:\mysql-{}-winx64\lib", version);
            let bin_dir = format!(r"C:\mysql-{}-winx64\bin", version);

            // Actualizar las variables de entorno en Windows
            if let Err(e) = set_environment_variable("MYSQLCLIENT_LIB_DIR", &lib_dir) {
                eprintln!("Error al configurar MYSQLCLIENT_LIB_DIR: {}", e);
            }

            if let Err(e) = set_environment_variable("MYSQLCLIENT_VERSION", &version) {
                eprintln!("Error al configurar MYSQLCLIENT_VERSION: {}", e);
            }

            if let Err(e) = update_path_variable(&bin_dir) {
                eprintln!("Error al actualizar el PATH: {}", e);
            }

            println!("✅ Variables de entorno actualizadas correctamente.");
            break;
        }
    }
}

fn desinstalar() {
    loop {
        println!("¿Está seguro que desea desinstalar? (y/n):");

        let mut confirmacion = String::new();
        io::stdin()
            .read_line(&mut confirmacion)
            .expect("Error al leer la confirmación");

        match confirmacion.trim().to_lowercase().as_str() {
            "y" => {
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

                println!("✅ Variables de entorno eliminadas correctamente.");
                break;
            }
            "n" => {
                println!("Desinstalación cancelada.");
                break;
            }
            _ => println!("Opción no válida, por favor ingrese 'y' (sí) o 'n' (no)."),
        }
    }
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

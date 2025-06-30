use std::env;

static BUILTINS: [&str; 5] = ["type", "echo", "exit", "pwd", "cd"];

pub fn echo(message: &str) {
    println!("{}", message);
}

pub fn pwd() {
    let current_dir = env::current_dir().unwrap();
    let pwd = current_dir.to_string_lossy();
    println!("{}", pwd);
}

pub fn cd(new_path: &str) {
    if new_path.is_empty() {
        println!("cd: missing argument");
        return;
    }
    if new_path == "~" {
        // Change to home directory
        if let Some(home) = env::var_os("HOME") {
            match env::set_current_dir(&home) {
                Ok(_) => {
                    let current_dir = env::current_dir().unwrap();
                    let pwd = current_dir.to_string_lossy();
                    println!("changed to {}", pwd);
                }
                Err(e) => {
                    println!("cd: {}: {}", home.to_string_lossy(), e);
                }
            }
            return;
        } else {
            println!("cd: HOME not set");
            return;
        }
    }
    match env::set_current_dir(new_path) {
        Ok(_) => {
            let current_dir = env::current_dir().unwrap();
            let pwd = current_dir.to_string_lossy();
            println!("changed to {}", pwd);
        }
        Err(e) => {
            println!("cd: {}: {}", new_path, e);
        }
    }
}

pub fn type_command(command: &str) -> String {
    if BUILTINS.contains(&command) {
        println!("{} is a shell builtin", command);
        "builtin".to_string()
    } else {
        if let Ok(global_path) = env::var("PATH") {
            let paths = global_path.split(';');
            for path_user in paths {
                let folders: Vec<&str> = path_user.split('\\').collect();
                let mut folder_path = String::from("C:");
                for folder in &folders[1..] {
                    folder_path.push_str("\\");
                    folder_path.push_str(folder);

                    if let Ok(entries) = std::fs::read_dir(folder_path.clone()) {
                        for entry in entries {
                            let entry = entry.unwrap();
                            let path = entry.path();
                            if path.is_file() && path.file_name().unwrap().to_string_lossy().trim_end_matches(".exe") == command {
                                println!("{} is {}", command, path.display());
                                return path.display().to_string();
                            }
                        }
                    } else {
                        // println!("Failed to read directory: {}", folder_path);
                    }
                }
            }
            println!("{}: Command not found", command);
            "not found".to_string()
        } else {
            println!("PATH variable not found");
            "path wrong".to_string()
        }
    }
}

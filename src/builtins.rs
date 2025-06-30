use std::env;

static BUILTINS: [&str; 3] = ["type", "echo", "exit"];

pub fn echo(message: &str) {
    println!("{}", message);
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

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct AppData {
    pub todo: Vec<String>,
    pub doing: Vec<String>,
    pub done: Vec<String>,
}

fn get_config_dir() -> std::path::PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("todosh");
    path
}

pub fn save_tasks(todo_list: &[String], doing_list: &[String], done_list: &[String]) {
    let data = AppData {
        todo: todo_list.to_vec(),
        doing: doing_list.to_vec(),
        done: done_list.to_vec(),
    };

    let path = get_config_dir();

    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create config directory");
    }

    let file_path = path.join("tasks.json");

    let json = serde_json::to_string_pretty(&data).expect("Failed to serialize data");
    std::fs::write(file_path, json).expect("Failed to write file");
}

pub fn load_tasks() -> AppData {
    let path = get_config_dir().join("tasks.json");

    match std::fs::read_to_string(path) {
        Ok(contents) => {
            serde_json::from_str(&contents).unwrap_or_else(|_| AppData::default())
        }
        Err(_) => {
            AppData::default()
        }
    }
}
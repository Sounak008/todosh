use serde::{Serialize, Deserialize};
const FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize, Default)]
pub struct AppData {
    pub todo: Vec<String>,
    pub doing: Vec<String>,
    pub done: Vec<String>,
}

pub fn save_tasks(todo_list: &[String], doing_list: &[String], done_list: &[String]) {
    let data = AppData {
        todo: todo_list.to_vec(),
        doing: doing_list.to_vec(),
        done: done_list.to_vec(),
    };

    let json = serde_json::to_string_pretty(&data).expect("Failed to serialize data");
    std::fs::write(FILE_PATH, json).expect("Failed to write file");
}

pub fn load_tasks() -> AppData {
    match std::fs::read_to_string(FILE_PATH) {
        Ok(contents) => {
            serde_json::from_str(&contents).unwrap_or_else(|_| AppData::default())
        }
        Err(_) => {
            AppData::default()
        }
    }
}
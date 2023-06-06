mod err;
use err::{ ParseErr, ReadErr };

pub use json::{parse, stringify};
pub use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = match File::open(path) {
            Err(er) => return Err(Box::new(ReadErr {child_err: Box::new(er)})),
            Ok(fs) => fs,
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Impossible de lire le fichier");
    
        if contents.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let json_data = match json::parse(&contents) {
            Err(er) => return Err(Box::new(er)),
            Ok(data) => data,
        };
        let title = json_data["title"].as_str() 
        .expect("Titre manquant dans le JSON")
        .to_owned();
        let tasks = json_data["tasks"]
        .members()
        .map(|task_json| {
            Task {
                id: task_json["id"].as_u32().expect("ID manquant dans le JSON"),
                description: task_json["description"]
                    .as_str()
                    .expect("Description manquante dans le JSON")
                    .to_owned(),
                level: task_json["level"].as_u32().expect("Niveau manquant dans le JSON"),
            }
        })
        .collect();

    let todo_list = TodoList { title, tasks };

        // println!("{:?}", todo_list);
        
        Ok(
            todo_list
        )
    }
}


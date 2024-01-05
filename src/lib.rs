use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};

pub const TODO_PATH: &str = "./to.do";

fn done_char(done: bool) -> char {
    if done == true {
        return '*'
    } else {
        return ' '
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: usize,
    msg: String,
    done: bool,
}

impl Todo {
    // list item
    pub fn list_item(&mut self) {
        println!("{} - [{}] {}", self.id, done_char(self.done), self.msg);
    }

    // initialize to.do file
    pub fn init() {
        if Path::new(TODO_PATH).exists() {
            println!("Whoops, to.do file already exists");
        } else {
            File::create(TODO_PATH).expect("Uh oh, to.do file creation failed");
            println!("Created to.do file");
        }
    }

    // add item to to.do file
    pub fn add(item: &Todo) {
        if !Path::new(TODO_PATH).exists() {
            println!("Please initialize a to.do file with 'td init'");
        } else {
            let mut data = OpenOptions::new()
                .append(true)
                .open(TODO_PATH)
                .expect("Cannot open to.do file");

            let serialized = serde_json::to_string(&item).unwrap();

            data.write(serialized.as_bytes()).expect("Failed to write to to.do file");
        }
    }
}

pub fn list(file: Vec<Todo>) {
    for mut item in file {
        Todo::list_item(&mut item);
    }
}


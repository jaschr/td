use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};

const TODO_PATH: &str = "./to.do";

fn done_char(done: bool) -> char {
    if done == true {
        return '*'
    } else {
        return ' '
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    msg: String,
    done: bool,
}

impl Todo {
    fn list(&mut self) {
        println!("{} - [{}] {}", self.id, done_char(self.done), self.msg);
    }

    fn init() {
        if Path::new(TODO_PATH).exists() {
            println!("Whoops, to.do file already exists");
        } else {
            File::create(TODO_PATH).expect("Uh oh, to.do file creation failed");
            println!("Created to.do file");
        }
    }

    fn add(item: Todo) {
        if !Path::new(TODO_PATH).exists() {
            println!("Please initialize a to.do file with 'td init'");
        } else {
            let mut data = OpenOptions::new()
                .append(true)
                .open(TODO_PATH)
                .expect("Cannot open to.do file");

            let sj = serde_json::to_string(&item).unwrap();

            data.write(sj.as_bytes()).expect("Failed to write to to.do file");
        }
    }
}

fn main() {
    Todo::init();

    let item1: Todo = Todo{ id: 1, msg: "Test item 1".to_owned(), done: true};
    let item2: Todo = Todo{ id: 2, msg: "Test item 2".to_owned(), done: true};
    let item3: Todo = Todo{ id: 2, msg: "Read to.do file".to_owned(), done: false};


    Todo::add(item1);
    Todo::add(item2);
    Todo::add(item3);

}

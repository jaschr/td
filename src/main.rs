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
    // list item
    fn list_item(&mut self) {
        println!("{} - [{}] {}", self.id, done_char(self.done), self.msg);
    }

    // initialize to.do file
    fn init() {
        if Path::new(TODO_PATH).exists() {
            println!("Whoops, to.do file already exists");
        } else {
            File::create(TODO_PATH).expect("Uh oh, to.do file creation failed");
            println!("Created to.do file");
        }
    }

    // add item to to.do file
    fn add(item: &Todo) {
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

fn main() {
    // initialize to.do file
    Todo::init();

    // test items
    let item1: Todo = Todo{ id: 1, msg: "Test item 1".to_owned(), done: true};
    let item2: Todo = Todo{ id: 2, msg: "Test item 2".to_owned(), done: true};
    let item3: Todo = Todo{ id: 3, msg: "Read to.do file".to_owned(), done: false};

    // add items to to.do file
    Todo::add(&item1);
    Todo::add(&item2);
    Todo::add(&item3);

    // create a vector of todos and push items to the vector
    let mut todos: Vec<Todo> = Vec::new();
    todos.push(item1);
    todos.push(item2);
    todos.push(item3);

    // loop over items in vector and list them 
    for i in 0..todos.len() {
        Todo::list_item(&mut todos[i]);
    }
}

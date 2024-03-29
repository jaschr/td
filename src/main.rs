use std::fs;

use clap::{arg, command, Command};
use td::*;

/*
fn deserialize(file: &str) -> Vec<Todo> {
    let mut todos: Vec<Todo> = Vec::new();
    if Path::new(TODO_PATH).exists() {
        let json = fs::read_to_string(file).expect("Failed to read to.do file");
        if file.len() != 0 {
            todos = serde_json::from_str(&json).unwrap();
        }
    }
    return todos;
}
*/

fn main() {
    let json = fs::read_to_string(TODO_PATH).expect("Failed to read to.do file");
    let mut todos: Todos = serde_json::from_str(&json).unwrap();

    // cli commands
    let matches = command!()
        .subcommand(
            Command::new("init")
                .about("Initialize to.do file")
        )
        .subcommand(
            Command::new("list")
                .about("List current todos")
        )
        .subcommand(
            Command::new("add")
            .about("Add a new item to the list")
            .arg(
                arg!(<item> "Todo item content")
            )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => Todo::init(),
        Some(("list", _)) => list(&mut todos.items),
        Some(("add", item)) => add_item(item.get_one::<String>("item").unwrap(), &mut todos.items),
        _ => list(&mut todos.items),
    }
    /*
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
    */
}

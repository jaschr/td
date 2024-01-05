use clap::{command, Command};

use td::*;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("init")
                .about("Initialize to.do file")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => Todo::init(),
        _ => {}
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

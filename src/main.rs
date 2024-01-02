
fn done_char(done: bool) -> char {
    if done == true {
        return '*'
    } else {
        return ' '
    }
}

struct Todo {
    id: usize,
    msg: String,
    done: bool,
}

impl Todo {
    fn list(&mut self) {
        println!("{} - [{}] {}", self.id, done_char(self.done), self.msg);
    }
}

fn main() {
    let item1: Todo = Todo{ id: 1, msg: "Test item 1".to_string(), done: true};
    let item2: Todo = Todo{ id: 2, msg: "Test item 2".to_string(), done: true};
    let item3: Todo = Todo{ id: 2, msg: "Read to.do file".to_string(), done: false};
    let mut todos: Vec<Todo> = Vec::new();
    todos.insert(0, item1);
    todos.insert(1, item2);
    todos.insert(2, item3);
    for i in 0..todos.len() {
        Todo::list(&mut todos[i]);
    }
}

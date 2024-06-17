mod todo;

use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use todo::Todo;

const TODO_FILE: &str = "todos.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: todo_cli <command> [<args>]");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            let text = args[2..].join(" ");
            add_todo(text);
        }
        "remove" => {
            let index: usize = args[2].parse().expect("Invalid index");
            remove_todo_by_index(index);
        }
        "complete" => {
            let index: usize = args[2].parse().expect("Invalid index");
            mark_todo_completed(index);
        }
        "complete_all" => mark_all_todos_completed(),
        "remove_all" => remove_all_todos(),
        "list" => list_all_todos(),
        "list_completed" => list_completed_todos(),
        "list_incomplete" => list_incomplete_todos(),
        _ => eprintln!("Unknown command: {}", command),
    }
}

fn load_todos() -> Vec<Todo> {
    if Path::new(TODO_FILE).exists() {
        let data = fs::read_to_string(TODO_FILE).expect("Unable to read file");
        serde_json::from_str(&data).expect("Unable to parse JSON")
    } else {
        vec![]
    }
}

fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string(todos).expect("Unable to serialize data");
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(TODO_FILE).expect("Unable to open file");
    file.write_all(data.as_bytes()).expect("Unable to write data");
}

fn add_todo(text: String) {
    let mut todos = load_todos();
    todos.push(Todo::new(text));
    save_todos(&todos);
}

fn remove_todo_by_index(index: usize) {
    let mut todos = load_todos();
    if index < todos.len() {
        todos.remove(index);
        save_todos(&todos);
    } else {
        eprintln!("Invalid index");
    }
}

fn mark_todo_completed(index: usize) {
    let mut todos = load_todos();
    if index < todos.len() {
        todos[index].completed = true;
        save_todos(&todos);
    } else {
        eprintln!("Invalid index");
    }
}

fn mark_all_todos_completed() {
    let mut todos = load_todos();
    for todo in &mut todos {
        todo.completed = true;
    }
    save_todos(&todos);
}

fn remove_all_todos() {
    let todos: Vec<Todo> = vec![];
    save_todos(&todos);
}

fn list_all_todos() {
    let todos = load_todos();
    for (index, todo) in todos.iter().enumerate() {
        let status = if todo.completed { "✓" } else { " " };
        println!("{}: [{}] {}", index, status, todo.text);
    }
}

fn list_completed_todos() {
    let todos = load_todos();
    for (index, todo) in todos.iter().enumerate().filter(|(_, t)| t.completed) {
        println!("{}: [✓] {}", index, todo.text);
    }
}

fn list_incomplete_todos() {
    let todos = load_todos();
    for (index, todo) in todos.iter().enumerate().filter(|(_, t)| !t.completed) {
        println!("{}: [ ] {}", index, todo.text);
    }
}

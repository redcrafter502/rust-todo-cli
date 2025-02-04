use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use prettytable::{row, Table};

fn main() {
    let input: Vec<String> = std::env::args().collect();

    println!("Input: {:?}", input);

    match input.len() {
        1 => println!("No arguments provided"),
        2 => one_argument(input[1].as_str()),
        3 => two_arguments(input[1].as_str(), input[2].as_str()),
        _ => println!("Too many arguments provided"),
    }
}

fn one_argument(argument: &str) {
    match argument {
        "--version" => print_version(),
        "-v" => print_version(),
        "--help" => print_default_help(),
        "list" => list_command(ListCommandFilter::Everything),
        _ => println!("Unknown command: {}", argument),
    }
}

fn two_arguments(first_argument: &str, second_argument: &str) {
    match (first_argument, second_argument) {
        ("list", "--help") => println!("HELP: TODO"),
        ("list", "--done") => list_command(ListCommandFilter::Done),
        ("list", "--undone") => list_command(ListCommandFilter::Undone),
        //("list", _) => list_command(ListCommandFilter::Everything),
        ("add", "--help") => println!("HELP: TODO"),
        ("add", _) => add_command(second_argument),
        ("remove", "--help") => println!("HELP: TODO"),
        ("remove", _) => remove_command(second_argument),
        //("--version", _) => print_version(),
        //("-v", _) => print_version(),
        //("--help", _) => print_default_help(),
        _ => println!("Unknown command: {} {}", first_argument, second_argument),
    }
}

fn print_version() {
    println!("Version: 0.1.0");
}

fn print_default_help() {
    println!("Usage: todos [command] [arguments]");
}

enum ListCommandFilter {
    Everything,
    Done,
    Undone,
}

struct Todo {
    id: i32,
    done: bool,
    content: String,
}

fn list_command(filter: ListCommandFilter) {
    match filter {
        ListCommandFilter::Everything => {
            let todos = get_todos_from_file();
            display_todos_in_table(todos);
        }
        ListCommandFilter::Done => println!("list --done"),
        ListCommandFilter::Undone => println!("list --undone"),
    }
}

fn display_todos_in_table(todos: Vec<Todo>) {
    let mut table = Table::new();
    table.add_row(row!["ID", "DONE", "CONTENT"]);
    for todo in todos {
        table.add_row(row![todo.id, todo.done, todo.content]);
    }
    table.printstd();
}

fn get_todos_from_file() -> Vec<Todo> {
    let file = File::open("todos.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let todos: Vec<Todo> = reader
        .lines()
        .filter_map(|line| match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 3 {
                    let id = parts[0].parse::<i32>().expect("Could not parse id");
                    let done = match parts[1] {
                        "0" => false,
                        "1" => true,
                        _ => false,
                    };
                    let content = parts[2..(parts.len())].join(":").to_string();
                    Some(Todo { id, done, content })
                } else {
                    eprintln!("Skipping line due to insufficient parts: {}", line);
                    None
                }
            }
            Err(error) => {
                eprintln!("Could not read line: {}", error);
                None
            }
        })
        .collect();
    return todos;
}

fn add_command(todo_content: &str) {
    let mut id = 1;

    let file = File::open("todos.txt")
        .or_else(|_| {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open("todos.txt")
        })
        .expect("Could not open file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.split(':').collect();
        let current_id = parts[0].parse::<i32>().expect("Could not parse id");
        if current_id >= id {
            id = current_id + 1;
        }
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todos.txt")
        .expect("Could not open file");

    writeln!(file, "{}:0:{}", id, todo_content).expect("Could not write to file");
    println!("Created Todo: {}", todo_content);
}

fn remove_command(todo_id: &str) {
    println!("remove {}", todo_id);
}

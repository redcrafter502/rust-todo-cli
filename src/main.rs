mod commands;
mod help;
use commands::{
    add_command, list_command, mark_done_command, mark_undone_command, remove_command,
    ListCommandFilter,
};
use help::{
    print_add_help, print_default_help, print_done_help, print_list_help, print_remove_help,
    print_undone_help,
};
use std::env::args;

fn main() {
    let input: Vec<String> = args().collect();
    match input.len() {
        1 => println!("No arguments provided"),
        2 => parse_arguments(input[1].as_str(), None),
        3 => parse_arguments(input[1].as_str(), Some(input[2].as_str())),
        _ => println!("Too many arguments provided"),
    }
}

fn parse_arguments(first_argument: &str, second_argument: Option<&str>) {
    match (first_argument, second_argument) {
        ("list", Some("--help")) => print_list_help(),
        ("list", Some("--done")) => list_command(ListCommandFilter::Done),
        ("list", Some("--undone")) => list_command(ListCommandFilter::Undone),
        ("list", None) => list_command(ListCommandFilter::Everything),
        ("add", Some("--help")) => print_add_help(),
        ("add", Some(todo_content)) => add_command(todo_content),
        ("done", Some("--help")) => print_done_help(),
        ("done", Some(todo_id)) => mark_done_command(todo_id),
        ("undone", Some("--help")) => print_undone_help(),
        ("undone", Some(todo_id)) => mark_undone_command(todo_id),
        ("remove", Some("--help")) => print_remove_help(),
        ("remove", Some(todo_id)) => remove_command(todo_id),
        ("--version", None) => print_version(),
        ("-v", None) => print_version(),
        ("--help", None) => print_default_help(),
        (_, Some(thing)) => println!("Unknown command: {} {}", first_argument, thing),
        (_, None) => println!("Unknown command: {}", first_argument),
    }
}

fn print_version() {
    println!("Version: 0.1.0");
}

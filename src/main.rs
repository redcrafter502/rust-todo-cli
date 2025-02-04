mod commands;
use commands::{
    add_command, list_command, mark_done_command, mark_undone_command, remove_command,
    ListCommandFilter,
};
use std::env::args;

fn main() {
    let input: Vec<String> = args().collect();

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
        ("done", "--help") => println!("HELP: TOOD"),
        ("done", _) => mark_done_command(second_argument),
        ("undone", "--help") => println!("HELP: TODO"),
        ("undone", _) => mark_undone_command(second_argument),
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

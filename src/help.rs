pub fn print_default_help() {
    println!("Usage: todos [command] [arguments]");
}

pub fn print_list_help() {
    println!("list: lists all of your todos and optionally filters them by done status");
    println!("Usage: todos list");
    println!("Usage: todos list --done");
    println!("Usage: todos list --undone");
}

pub fn print_add_help() {
    println!("add: adds a todo to your list of todos");
    println!("Usage: todos add [content of your todo]");
}

pub fn print_done_help() {
    println!("done: marks a todo as done");
    println!("Usage: todos done [id of the todo]")
}

pub fn print_undone_help() {
    println!("done: marks a todo as undone");
    println!("Usage: todos undone [id of the todo]");
}

pub fn print_remove_help() {
    println!("remove: removes an todo from your list");
    println!("Usage: todos remove [id of the todo]")
}

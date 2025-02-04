# Rust Todo Cli

## Commands
- `todos add <todo-content>`
- `todos list`
- `todos list --done`
- `todos list --undone`
- `todos done <id>`
- `todos undone <id>`
- `todos remove <id>`

## Storage format
`<id>:<done>:<todo-content>`
Example:
`1:0:Wash dishes` - A todo with the id 1 marked as undone
`2:1:Drink water` - A todo with the id 2 marked as done

## Todo
- [x] Add todos
- [x] List todos
- [x] List only done todos
- [x] List only undone todos
- [ ] Mark todos as done
- [ ] Mark todos as undone
- [ ] Delete a todo from the list
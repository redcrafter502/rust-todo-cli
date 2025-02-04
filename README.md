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
- [ ] Add todos
- [ ] List todos
- [ ] List only done todos
- [ ] List only undone todos
- [ ] Mark todos as done
- [ ] Mark todos as undone
- [ ] Delete a todo from the list
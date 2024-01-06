# td - cli todos

## Commands

### init

Usage: `td init`

Creates a to.do file.

### list

Usage: `td list`

Lists all todos in the to.do file.

### add

Usage: `td add <item>`

Add a new todo item to the to.do file

### rm

Usage: `td rm <id>`

Remove the todo item at the specified ID.

### done

Usage: `td done <id>`

Marks the todo item at the specified ID as completed.

### clean

Removes all items from the to.do file.

## Issues

- `td add` needs to append a new item to the items Vec<Todo> of the Todos struct.

- Ensuring **to.do* file exists and **Todos** is not empty prior to deserializing file not yet implemented.

- Serializing/Deserializing data properly into/from structs.

- Commands `td rm`, `td done`, and `td clean` not yet implemented.

- Unit tests not yet implemented.

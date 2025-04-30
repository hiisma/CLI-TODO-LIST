# CLI TODO LIST
The objective of this project is to start learning rust by creating
a todo list cli application.

## How to run.
Execute `cargo run` and you should be ready.
If you want the realse version just run `cargo build --release`

## Notes:
The application will generate a tasks.json file if not found.
If you want a file to be readed you will need to have it in the same directory
as the executable.

### JSON structure.
It will read a list of tasks which are encoded in the following way:
```json
[
  {
    "name":"Do dishes",
    "text":"Before 20:30"
  },
  {
    "name":"Walk the dog",
    "text":"Last time she peed in the couch :("
  },
]
```

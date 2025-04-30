use std::io::{self, Write};
use std::fs::{self, File};
use serde::{Serialize, Deserialize};

const ERR_FLUSH_MSG: &str = "Failed to STD Flush";
const ERR_WFILE_MSG: &str = "Failed to write file";
const ERR_RFILE_MSG: &str = "Failed to read file";
const ERR_STDIN_MSG: &str = "Failed to read input";
const JSON_PATH: &str = "tasks.json";

#[derive(Serialize,Deserialize,Debug)]
struct Task {
    name: String,
    text: String,
}

fn list_tasks(v: &Vec<Task>) {
    println!("====================Tasks======================");
    for (i, task) in v.iter().enumerate() {
        println!("=== Task ===");
        println!("ID: {}", i);
        println!("Title: {}", task.name);
        println!("{}", task.text);
        println!("============");
    }
    println!("");
}

fn add_task(v: &mut Vec<Task>) {
    println!("ADDING TASK");

    print!("TITLE: ");
    io::stdout().flush().expect(ERR_FLUSH_MSG);
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect(ERR_STDIN_MSG);

    print!("TEXT: ");
    io::stdout().flush().expect(ERR_FLUSH_MSG);
    let mut text: String = String::new();
    io::stdin().read_line(&mut text).expect(ERR_STDIN_MSG);

    let name = name.trim().to_string();
    let text = text.trim().to_string();

    let task: Task = Task { name, text };

    v.push(task);
}

fn remove_task(v: &mut Vec<Task>) {
    print!("Task to delete no.: ");
    io::stdout().flush().expect(ERR_FLUSH_MSG);
    let mut input: String = String::new();

    if io::stdin().read_line(&mut input).is_err() {
        println!("{}", ERR_STDIN_MSG);
        return;
    }

    let unwraped_index = input.trim().parse::<usize>();
    match unwraped_index {
        Ok(n) => {
            if n >= v.len() {
                println!("Task does not exist");
            } else {
                v.remove(n);
            }
        }
        Err(_) => println!("NaN: Your input wasn't a number or it was negative"),
    }
}

fn show_help() {
    println!("===========HELP==========");
    println!("x / exit       -> exit");
    println!("l / ls / list  -> list tasks");
    println!("a / add        -> add tasks");
    println!("r / remove /rm -> remove task");
    println!("h / help       -> show help");
    println!("c / clear      -> clear screen");
    println!("=========================");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().expect(ERR_FLUSH_MSG);
}

fn load_json() -> Vec<Task> {
    if let Ok(json) = fs::read_to_string(JSON_PATH) {
        serde_json::from_str(&json).unwrap_or_else( |_| {
            eprintln!("Failed to parse JSON, starting with empty list");
            Vec::new()
        })
    } else {
        Vec::new()
    }
}

fn save_json(v: Vec<Task>) {
    let json = serde_json::to_string(&v).unwrap();
    fs::write(JSON_PATH, json).expect(ERR_WFILE_MSG);
}

fn main() {
    // Initialization
    let mut tasks = load_json();
    // Menu

    show_help();
    loop {
        print!("Tasks > ");
        io::stdout().flush().expect(ERR_FLUSH_MSG);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect(ERR_STDIN_MSG);

        let command = user_input.trim();
        match command {
            "x" | "exit" => {
                save_json(tasks);
                return;
            },
            "l" | "ls" | "list" => list_tasks(&tasks),
            "a" | "add" => add_task(&mut tasks),
            "r" | "rm" | "remove" => remove_task(&mut tasks),
            "h" | "help" => show_help(),
            "c" | "clear" => clear_screen(),
            _ => println!("No such command"),
        }
    }
}

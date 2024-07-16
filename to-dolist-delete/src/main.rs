use clap::{Parser, Subcommand};
use std::fs::{read_to_string, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser)]
#[command(name = "Todo")]
#[command(about = "A simple to-do list CLI app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg()]
        task: String,
    },
    Delete {
        #[arg()]
        task_number: usize,
    },
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { task } => add_task(&task),
        Commands::Delete { task_number } => delete_task(*task_number),
        Commands::List => list_tasks(),
    }
}

fn add_task(task: &str) {
    let file_path = Path::new("todo.txt");
    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            eprintln!("Current directory: {:?}", std::env::current_dir().unwrap_or_default());
            return;
        }
    };

    if let Err(e) = writeln!(file, "{}", task) {
        eprintln!("Failed to write to file: {}", e);
    } else {
        println!("Added task: {}", task);
    }
}

fn delete_task(task_number: usize) {
    let content = read_to_string("todo.txt").expect("Failed to read file");
    let mut tasks: Vec<&str> = content.lines().collect();

    if task_number >= tasks.len() {
        eprintln!("Invalid task number.");
        return;
    }

    tasks.remove(task_number);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("todo.txt")
        .expect("Failed to open file");

    for task in tasks {
        writeln!(file, "{}", task).expect("Failed to write to file");
    }

    println!("Deleted task number: {}", task_number);
}



fn list_tasks() {
    let content = read_to_string("todo.txt").expect("Failed to read file");

    if content.is_empty() {
        println!("No tasks found.");
    } else {
        for (index, task) in content.lines().enumerate() {
            println!("{}: {}", index, task);
        }
    }
}

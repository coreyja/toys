use clap::{Parser, Subcommand};
use std::fs::{OpenOptions, read_to_string, File};
use std::io::{Write, Error};

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
        Commands::Add { task } => {
            if let Err(e) = add_task(task) {
                eprintln!("Error adding task: {}", e);
            }
        },
        Commands::Delete { task_number } => {
            if let Err(e) = delete_task(*task_number) {
                eprintln!("Error deleting task: {}", e);
            }
        },
        Commands::List => list_tasks(),
    }
}

fn add_task(task: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt")?;
    writeln!(file, "{}", task)?;
    println!("Added task: {}", task);
    Ok(())
}

fn delete_task(task_number: usize) -> Result<(), Error> {
    let content = read_to_string("todo.txt")?;
    let mut tasks: Vec<&str> = content.lines().collect();

    if task_number > tasks.len() {
        eprintln!("Invalid task number.");
        return Ok(());
    }

    tasks.remove(task_number);

    let mut file = File::create("todo.txt")?;
    for task in tasks {
        writeln!(file, "{}", task)?;
    }

    println!("Deleted task number: {}", task_number);
    Ok(())
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

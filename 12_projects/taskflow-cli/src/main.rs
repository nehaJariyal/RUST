mod models;
mod storage;

use clap::{Parser, Subcommand, ValueEnum};
use models::{Priority, TaskStore};

#[derive(Parser)]
#[command(
    name = "taskflow",
    about = "CLI Task Manager - Rust CV project",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task title
        title: String,
        /// Priority: low, medium, high
        #[arg(short, long, default_value = "medium")]
        priority: CliPriority,
    },
    /// Show all tasks
    List,
    /// Mark a task as complete
    Done {
        /// Task id
        id: u32,
    },
    /// Delete a task
    Delete {
        /// Task id
        id: u32,
    },
    /// Remove completed tasks
    Clear,
    /// Show how many pending tasks there are
    Stats,
}

#[derive(Clone, ValueEnum)]
enum CliPriority {
    Low,
    Medium,
    High,
}

impl From<CliPriority> for Priority {
    fn from(value: CliPriority) -> Self {
        match value {
            CliPriority::Low => Priority::Low,
            CliPriority::Medium => Priority::Medium,
            CliPriority::High => Priority::High,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let mut store = storage::load()?;

    match cli.command {
        Commands::Add { title, priority } => {
            store.add(title, priority.into());
            let task = store.tasks.last().expect("task just added");
            let (id, title, priority) = (task.id, task.title.clone(), task.priority.clone());
            storage::save(&store)?;
            println!("Task add ho gaya -> #{} [{}] {:?}", id, title, priority);
        }
        Commands::List => print_list(&store),
        Commands::Done { id } => {
            store.complete(id)?;
            storage::save(&store)?;
            println!("Task #{} complete mark ho gaya", id);
        }
        Commands::Delete { id } => {
            store.delete(id)?;
            storage::save(&store)?;
            println!("Task #{} delete ho gaya", id);
        }
        Commands::Clear => {
            let removed = store.clear_done();
            storage::save(&store)?;
            println!("{} completed tasks hata diye", removed);
        }
        Commands::Stats => {
            println!(
                "Total: {} | Pending: {} | Done: {}",
                store.tasks.len(),
                store.pending_count(),
                store.tasks.len() - store.pending_count()
            );
        }
    }

    Ok(())
}

fn print_list(store: &TaskStore) {
    if store.tasks.is_empty() {
        println!("Koi task nahi hai. Pehla add karo:");
        println!("  cargo run -- add \"Rust padhna\" --priority high");
        return;
    }

    println!("\n===== TASKFLOW LIST =====");
    for task in &store.tasks {
        let mark = if task.done { "[x]" } else { "[ ]" };
        println!(
            "{} #{} {} ({:?})",
            mark, task.id, task.title, task.priority
        );
    }
    println!("Pending: {}\n", store.pending_count());
}

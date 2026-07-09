// ============================================================
// PROJECT: To-Do List  (in-memory task manager)
// ============================================================
// Run:  rustc todo_list.rs && ./todo_list
// ------------------------------------------------------------
// Concepts used: struct, impl/methods, Vec, enum, Option, iterators, format.
// This shows all these concepts together in a small real project.

// Priority of a task
#[derive(Debug, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

// A single task
struct Task {
    id: u32,
    title: String,
    done: bool,
    priority: Priority,
}

// The full to-do list (collection of tasks)
struct TodoList {
    tasks: Vec<Task>,
    next_id: u32, // what id to assign next
}

impl TodoList {
    // Creates a new empty list
    fn new() -> TodoList {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Adds a new task
    fn add(&mut self, title: &str, priority: Priority) {
        let task = Task {
            id: self.next_id,
            title: String::from(title),
            done: false,
            priority,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }

    // Marks a task as "done" by id
    fn complete(&mut self, id: u32) {
        // iter_mut -> for mutating tasks
        for task in self.tasks.iter_mut() {
            if task.id == id {
                task.done = true;
                return;
            }
        }
        println!("(id {} ka koi task nahi mila)", id);
    }

    // Prints all tasks
    fn show(&self) {
        println!("\n===== MERI TO-DO LIST =====");
        if self.tasks.is_empty() {
            println!("(list khaali hai)");
            return;
        }
        for task in &self.tasks {
            // [x] if done, [ ] otherwise
            let mark = if task.done { "[x]" } else { "[ ]" };
            println!("{} #{} {} ({:?})", mark, task.id, task.title, task.priority);
        }
    }

    // How many tasks are left (pending) - counts them
    fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.done).count()
    }
}

fn main() {
    let mut list = TodoList::new();

    // Add some tasks
    list.add("Rust padhna", Priority::High);
    list.add("Khana banana", Priority::Medium);
    list.add("Movie dekhna", Priority::Low);
    list.add("Exercise karna", Priority::High);

    list.show();
    println!("Pending tasks: {}", list.pending_count());

    // Complete some tasks
    println!("\n>> Task #1 aur #3 complete kar rahe hai...");
    list.complete(1);
    list.complete(3);

    list.show();
    println!("Pending tasks: {}", list.pending_count());

    // Find high priority pending tasks (filter example)
    println!("\n>> High priority pending tasks:");
    for task in list.tasks.iter().filter(|t| !t.done && t.priority == Priority::High) {
        println!("  - {}", task.title);
    }
}

// ============================================================
// PROJECT: To-Do List  (in-memory task manager)
// ============================================================
// Run:  rustc todo_list.rs && ./todo_list
// ------------------------------------------------------------
// Use kiya: struct, impl/methods, Vec, enum, Option, iterators, format.
// Ye saare concepts ek chhote real project me ek saath dikhata hai.

// Ek task ki priority
#[derive(Debug, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

// Ek single task
struct Task {
    id: u32,
    title: String,
    done: bool,
    priority: Priority,
}

// Poori to-do list (tasks ka collection)
struct TodoList {
    tasks: Vec<Task>,
    next_id: u32, // agla id kya dena hai
}

impl TodoList {
    // Nayi khaali list banata hai
    fn new() -> TodoList {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Naya task add karta hai
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

    // id se task ko "done" mark karta hai
    fn complete(&mut self, id: u32) {
        // iter_mut -> tasks ko badalne ke liye
        for task in self.tasks.iter_mut() {
            if task.id == id {
                task.done = true;
                return;
            }
        }
        println!("(id {} ka koi task nahi mila)", id);
    }

    // Saari tasks print karta hai
    fn show(&self) {
        println!("\n===== MERI TO-DO LIST =====");
        if self.tasks.is_empty() {
            println!("(list khaali hai)");
            return;
        }
        for task in &self.tasks {
            // done hai to [x], warna [ ]
            let mark = if task.done { "[x]" } else { "[ ]" };
            println!("{} #{} {} ({:?})", mark, task.id, task.title, task.priority);
        }
    }

    // Kitne tasks baaki hai (pending) - count nikalta hai
    fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.done).count()
    }
}

fn main() {
    let mut list = TodoList::new();

    // Kuch tasks add karte hai
    list.add("Rust padhna", Priority::High);
    list.add("Khana banana", Priority::Medium);
    list.add("Movie dekhna", Priority::Low);
    list.add("Exercise karna", Priority::High);

    list.show();
    println!("Pending tasks: {}", list.pending_count());

    // Kuch tasks complete karte hai
    println!("\n>> Task #1 aur #3 complete kar rahe hai...");
    list.complete(1);
    list.complete(3);

    list.show();
    println!("Pending tasks: {}", list.pending_count());

    // High priority pending tasks dhoondhna (filter example)
    println!("\n>> High priority pending tasks:");
    for task in list.tasks.iter().filter(|t| !t.done && t.priority == Priority::High) {
        println!("  - {}", task.title);
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
    pub priority: Priority,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskStore {
    pub tasks: Vec<Task>,
    #[serde(default = "default_next_id")]
    pub next_id: u32,
}

fn default_next_id() -> u32 {
    1
}

impl Default for TaskStore {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }
}

impl TaskStore {
    pub fn add(&mut self, title: String, priority: Priority) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            done: false,
            priority,
        };
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().expect("task just pushed")
    }

    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        let task = self
            .find_mut(id)
            .ok_or_else(|| format!("Task #{} nahi mila", id))?;
        task.done = true;
        Ok(())
    }

    pub fn delete(&mut self, id: u32) -> Result<(), String> {
        let pos = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| format!("Task #{} nahi mila", id))?;
        self.tasks.remove(pos);
        Ok(())
    }

    pub fn clear_done(&mut self) -> usize {
        let before = self.tasks.len();
        self.tasks.retain(|t| !t.done);
        before - self.tasks.len()
    }

    pub fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.done).count()
    }

    fn find_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_complete_task() {
        let mut store = TaskStore::default();
        let task = store.add("Rust padhna".into(), Priority::High);
        assert_eq!(task.id, 1);
        assert!(!task.done);

        store.complete(1).unwrap();
        assert!(store.tasks[0].done);
    }

    #[test]
    fn delete_task() {
        let mut store = TaskStore::default();
        store.add("Test".into(), Priority::Low);
        store.delete(1).unwrap();
        assert!(store.tasks.is_empty());
    }
}

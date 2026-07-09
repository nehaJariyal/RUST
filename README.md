# Rust

## Running a Single Rust File

### Step 1: Compile the file

```bash
rustc filename.rs
rustc comments.rs
```

### Step 2: Run the executable

```bash
./filename
./comments
```

> **Note (Windows):**
>
> ```cmd
> filename.exe
> comment.exe
> ```

---

## Purpose

- **`rustc filename.rs`**
  - Compiles the Rust source code (`filename.rs`) into a native executable.
  - Checks for syntax and compilation errors.
  - Produces a binary file (`filename` on Linux/macOS or `filename.exe` on Windows).

- **`./filename`**
  - Executes the compiled Rust program.
  - Runs the `main()` function and displays the program's output.

  # Run a Rust Project

A Rust project is managed using **Cargo**, Rust's official build system and package manager.

## Step 1: Open the project directory

```bash
cd project_name
```

Example:

```bash
cd my_project
```

---

## Step 2: Run the project

```bash
cargo run
```

### Purpose

- Compiles the project (if needed).
- Downloads and builds dependencies from `Cargo.toml`.
- Runs the executable.
- Rebuilds only the files that have changed.

---

## Build the project without running

```bash
cargo build
```

### Purpose

- Compiles the project.
- Creates the executable in the `target/debug/` directory.
- Does not run the program.

---

## Build an optimized (Release) version

```bash
cargo build --release
```

### Purpose

- Builds the project with compiler optimizations.
- Produces a faster executable.
- Output is stored in `target/release/`.

---

## Run the optimized (Release) version

```bash
cargo run --release
```

### Purpose

- Builds the optimized version (if needed).
- Runs the release executable.
- Recommended for production or performance testing.

---

## Project Structure

```
my_project/
│
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs
└── target/
```

---

## Common Cargo Commands

| Command | Purpose |
|---------|---------|
| `cargo run` | Build and run the project |
| `cargo build` | Build the project only |
| `cargo build --release` | Build an optimized release version |
| `cargo run --release` | Run the optimized release version |
| `cargo check` | Check for compilation errors without building |
| `cargo test` | Run tests |
| `cargo fmt` | Format the source code |
| `cargo clippy` | Analyze the code for common mistakes |
| `cargo clean` | Remove all build artifacts |
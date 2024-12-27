# Dash Engine - Filesystem/Networking Utilities

A simple library crate that provides utility functions for interacting with the filesystem, including creating, reading, and deleting files. 

## Features

- **Create a file**: Create a file in a specific directory, creating the directory if it doesn't exist.
- **Read a file**: Read the contents of a file from a specific directory.
- **Delete a file**: Delete a file from a specific directory.

## Development Features

- **Download a file**: Download a file from the internet into an specific directory.

---

## Installation

To add this crate as a dependency in your project, update your `Cargo.toml` file:

```toml
[dependencies]
dash_rs = "0.1.0"
```

Replace `"0.1.0"` with the appropriate version if needed.

## Usage

You can call the functions from the `filesystem` module to interact with the filesystem. Below are examples for each function.

### Example 1: Create a File

To create a file, you can use the `create_file` function:

```rust
use dash_engine::filesystem::create_file;

fn main() {
    match create_file("path/to/directory", "example.txt") {
        Ok(()) => println!("File created successfully!"),
        Err(e) => println!("Error creating file: {}", e),
    }
}
```

### Example 2: Read a File

To read a file, use the `read_file` function:

```rust
use dash_engine::filesystem::read_file;

fn main() {
    match read_file("path/to/directory", "example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

### Example 3: Delete a File

To delete a file, use the `delete_file` function:

```rust
use dash_engine::filesystem::delete_file;

fn main() {
    match delete_file("path/to/directory", "example.txt") {
        Ok(()) => println!("File deleted successfully!"),
        Err(e) => println!("Error deleting file: {}", e),
    }
}
```

## Function Documentation

### `create_file`

```rust
pub fn create_file(directory: &str, file_name: &str) -> io::Result<()>;
```

#### Arguments:
- `directory`: The path to the directory where the file should be created.
- `file_name`: The name of the file to create.

#### Returns:
- `Ok(())` if the file is created successfully.
- `Err(io::Error)` if an error occurs.

---

### `read_file`

```rust
pub fn read_file(directory: &str, file_name: &str) -> io::Result<String>;
```

#### Arguments:
- `directory`: The path to the directory where the file is located.
- `file_name`: The name of the file to read.

#### Returns:
- `Ok(String)` containing the file's content.
- `Err(io::Error)` if an error occurs.

---

### `delete_file`

```rust
pub fn delete_file(directory: &str, file_name: &str) -> io::Result<()>;
```

#### Arguments:
- `directory`: The path to the directory where the file is located.
- `file_name`: The name of the file to delete.

#### Returns:
- `Ok(())` if the file is deleted successfully.
- `Err(io::Error)` if an error occurs.

---

## License

This project is licensed under the following license:

- MIT License

---

### Contributing

1. Fork the repository.
2. Clone your fork locally.
3. Create a new branch for your changes.
4. Make your changes and add tests.
5. Submit a pull request.

---

### Additional Notes

- Ensure the provided directory paths are correct and the application has appropriate permissions to read/write files.
- The `create_file` function creates the entire directory structure if it doesn't already exist (using `fs::create_dir_all`).
- The `read_file` function reads the entire file content into a `String`.
- The `delete_file` function removes the specified file.

---
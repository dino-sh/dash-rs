
/// This module provides utilities for interacting with the file system.
pub mod filesystem {
    use std::fs::{self, File};
    use std::io::{self, Read};
    use std::path::Path;

    /// Creates a file with a specific name in a specific directory.
    /// If the directory does not exist, it will be created.
    /// 
    /// # Arguments
    /// * `directory` - The path to the directory where the file should be created.
    /// * `file_name` - The name of the file to create.
    /// 
    /// # Returns
    /// * `Ok(())` if the file is created successfully.
    /// * `Err(io::Error)` if an error occurs.
    pub fn create_file(directory: &str, file_name: &str) -> io::Result<()> {
        let dir_path = Path::new(directory);

        if !dir_path.exists() {
            fs::create_dir_all(dir_path)?;
        }

        let file_path = dir_path.join(file_name);

        File::create(file_path)?;

        Ok(())
    }

    /// Reads the content of a file in a specific directory and returns it as a string.
    /// 
    /// # Arguments
    /// * `directory` - The path to the directory where the file is located.
    /// * `file_name` - The name of the file to read.
    /// 
    /// # Returns
    /// * `Ok(String)` containing the file's content.
    /// * `Err(io::Error)` if an error occurs.
    pub fn read_file(directory: &str, file_name: &str) -> io::Result<String> {
        let file_path = Path::new(directory).join(file_name);

        // Open the file and read its contents
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(content)
    }

    /// Deletes a file with a specific name in a specific directory.
    /// 
    /// # Arguments
    /// * `directory` - The path to the directory where the file is located.
    /// * `file_name` - The name of the file to delete.
    /// 
    /// # Returns
    /// * `Ok(())` if the file is deleted successfully.
    /// * `Err(io::Error)` if an error occurs.
    pub fn delete_file(directory: &str, file_name: &str) -> io::Result<()> {
        let file_path = Path::new(directory).join(file_name);

        // Delete the file
        fs::remove_file(file_path)?;

        Ok(())
    }

}
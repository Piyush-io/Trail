use colored::Colorize;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::io;
use trail::{
    create_trail_file, filter_todo_lines, find_rs_files, find_src_path, generate_buffer_content,
    read_file_content, write_to_file,
};

fn main() -> io::Result<()> {
    let mut source_file_paths: Vec<PathBuf> = Vec::new();
    let mut buffer_file_paths: Vec<PathBuf> = Vec::new();

    let trail_file_path = create_trail_file();
    let paths = read_file_content(&trail_file_path)?;

    for path in paths.lines() {
        let path = PathBuf::from(path);
        source_file_paths.push(path);
    }

    let combined_buffer = Arc::new(Mutex::new(String::new()));
    let mut count = Arc::new(Mutex::new(1));

    // Use Rayon for parallel processing of the source directories
    source_file_paths.par_iter().for_each(|path| {
        let parent_path = path.parent().unwrap();
        let src_path = find_src_path(&path).unwrap_or_else(|| {
            eprintln!("Cargo.toml not found! Returning current path.");
            path.to_path_buf()
        });

        let rs_files = find_rs_files(&src_path);
        let buffer_file_path = parent_path.join("buffer.txt");

        // Process each Rust file in parallel using Rayon
        rs_files.par_iter().for_each(|file| {
            if let Ok(content) = read_file_content(file) {
                let result = filter_todo_lines(&content);

                // Generate buffer content in parallel
                let mut local_buffer = String::new();
                let mut local_count = count.lock().unwrap().clone();

                let buffer = generate_buffer_content(file, &result, &mut local_count);

                // Update shared combined buffer
                let mut combined_buffer_guard = combined_buffer.lock().unwrap();
                combined_buffer_guard.push_str(&buffer);

                // Write the buffer to the corresponding file
                write_to_file(&buffer_file_path, &combined_buffer_guard).expect("Failed to write buffer");
            }
        });

        buffer_file_paths.push(buffer_file_path);
    });

    // Print the content of buffer files
    buffer_file_paths.iter().for_each(|buffer_file_path| {
        let buffer_content = read_file_content(buffer_file_path).expect("Failed to read buffer content");
        let buffer_file_dir = buffer_file_path.parent().unwrap();
        let buffer_file_dir_as_str = buffer_file_dir.to_string_lossy();
        println!(
            "Content of {}:\n{}",
            buffer_file_dir_as_str.bold().bright_white(),
            buffer_content.bright_cyan().bold()
        );
    });

    Ok(())
}

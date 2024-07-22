use colored::Colorize;
use std::io;
use std::path::{Path, PathBuf};
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

    let mut combined_buffer = String::new();
    let mut count = 1;

    for path in &source_file_paths {
        let parent_path = path.parent().unwrap();
        let src_path = find_src_path(&path).unwrap_or_else(|| {
            eprintln!("Cargo.toml not found! Returning current path.");
            path.to_path_buf()
        });

        let rs_files = find_rs_files(&src_path);
        let buffer_file_path = parent_path.join("buffer.txt");

        for file in &rs_files {
            let content = read_file_content(&file)?;
            let result = filter_todo_lines(&content);
            let buffer = generate_buffer_content(file, &result, &mut count);
            combined_buffer.push_str(&buffer);
        }

        write_to_file(&buffer_file_path, &combined_buffer)?;
        buffer_file_paths.push(buffer_file_path);
        combined_buffer.clear(); // Clear combined_buffer for the next directory
    }

    for buffer_file_path in &buffer_file_paths {
        let buffer_content = read_file_content(buffer_file_path)?;
        let buffer_file_dir = buffer_file_path.parent().unwrap();
        let buffer_file_dir_as_str = buffer_file_dir.to_string_lossy();
        println!(
            "Content of {}:\n{}",
            buffer_file_dir_as_str.bold().bright_white(),
            buffer_content.bright_cyan().bold()
        );
    }

    Ok(())
}

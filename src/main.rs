use std::{fmt::Write, fs, io /* process::Command*/};

fn read_file_content(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn filter_todo_lines(content: &str) -> Vec<(usize, &str)> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("TODO") || line.contains("todo"))
        .collect()
}

fn generate_buffer_content(path: &str, result: &[(usize, &str)], count: &mut usize) -> String {
    let mut buffer = String::new();
    for (line_no, line) in result {
        writeln!(
            &mut buffer,
            "{}. {}, line number {}, located at {}",
            *count,
            line.trim(),
            line_no + 1,
            path
        )
        .expect("Error writing to buffer");
        *count += 1;
    }
    buffer
}

fn write_to_file(file_path: &str, content: &str) -> io::Result<()> {
    fs::write(file_path, content)?;
    Ok(())
}

// fn execute_cat_command(file_path: &str) -> io::Result<()> {
//     let status = Command::new("cat")
//         .arg(file_path)
//         .status()
//         .expect("Failed to execute command");
//     if !status.success() {
//         println!("Failed to read buffer file");
//     }
//     Ok(())
// }

fn main() -> io::Result<()> {
    let mut paths: Vec<&String> = Vec::new();
    let path1 = "/Users/xhail/projects/minigrep/src/lib.rs".to_string();
    let path2 = "/Users/xhail/rustlings/exercises/20_threads/threads2.rs".to_string();
    let buffer_file_path = "/Users/xhail/Desktop/PR1/trail/buffer.txt".to_string();

    paths.push(&path1);
    paths.push(&path2);

    let mut combined_buffer = String::new();
    let mut count = 1;

    for path in &paths {
        // Read content from file
        let content = read_file_content(path)?; // We recieve Result Type

        // Filter lines containing TODO
        let result = filter_todo_lines(&content); // We send a String as Result automatically dereferences, we recieve a vector

        // Generate content for the buffer line
        let buffer = generate_buffer_content(path, &result, &mut count); // We send the path and the vector, and recieve the result as string
        combined_buffer.push_str(&buffer);
    }

    // Write buffer content to file
    write_to_file(&buffer_file_path, &combined_buffer)?; // Writing the content to buffer file

    // execute_cat_command(buffer_file_path)?;

    Ok(())
}

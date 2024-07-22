#!/bin/bash

# Paths to the scripts
BIN_DIR="$HOME/bin"
THIS_SCRIPT="$BIN_DIR/trail_this.sh"
END_SCRIPT="$BIN_DIR/end_trail.sh"
TODO_SCRIPT="$BIN_DIR/trail.sh"
TRAIL_FILE="$HOME/trail_directories.txt"

# Create the bin directory if it doesn't exist
mkdir -p "$BIN_DIR"

# Create trail_this.sh script
cat << 'EOF' > "$THIS_SCRIPT"
#!/bin/bash
TRAIL_FILE="$HOME/trail_directories.txt"

add_directory() {
    local dir
    dir=$(pwd)
    mkdir -p "$(dirname "$TRAIL_FILE")"
    if ! grep -qxF "$dir" "$TRAIL_FILE"; then
        echo "$dir" >> "$TRAIL_FILE"
        echo "Added $dir to the trail."
    else
        echo "$dir is already in the trail."
    fi
}

add_directory
EOF

# Create end_trail.sh script
cat << 'EOF' > "$END_SCRIPT"
#!/bin/bash
TRAIL_FILE="$HOME/trail_directories.txt"

remove_directory() {
    local dir
    dir=$(pwd)

    if [ -f "$TRAIL_FILE" ]; then
        if grep -qxF "$dir" "$TRAIL_FILE"; then
            # Create a new temporary file to store the updated content
            tmp_file=$(mktemp)
            # Remove the line containing the directory
            grep -vxF "$dir" "$TRAIL_FILE" > "$tmp_file"
            # Replace the original file with the updated file
            mv "$tmp_file" "$TRAIL_FILE"
            echo "Removed $dir from the trail."
        else
            echo "$dir is not in the trail."
        fi
    else
        echo "Trail file does not exist."
    fi
}

remove_directory
EOF

# Create trail.sh script
cat << 'EOF' > "$TODO_SCRIPT"
#!/bin/bash

# Path to the Rust executable
RUST_EXECUTABLE="/Users/xhail/Desktop/PR1/trail/target/release/trail"

if [ ! -x "$RUST_EXECUTABLE" ]; then
    echo "Rust executable not found. Please make sure the Rust program is compiled."
    exit 1
fi

# Run the Rust executable
"$RUST_EXECUTABLE"
EOF

# Make scripts executable
chmod +x "$THIS_SCRIPT"
chmod +x "$END_SCRIPT"
chmod +x "$TODO_SCRIPT"

# Add aliases to shell configuration
SHELL_CONFIG="$HOME/.zshrc"  # Update for zsh

if ! grep -q 'alias trail_this=' "$SHELL_CONFIG"; then
    echo "alias trail_this=\"$THIS_SCRIPT\"" >> "$SHELL_CONFIG"
fi

if ! grep -q 'alias end_trail=' "$SHELL_CONFIG"; then
    echo "alias end_trail=\"$END_SCRIPT\"" >> "$SHELL_CONFIG"
fi

if ! grep -q 'alias trail=' "$SHELL_CONFIG"; then
    echo "alias trail=\"$TODO_SCRIPT\"" >> "$SHELL_CONFIG"
fi

# Apply changes to shell configuration
source "$SHELL_CONFIG"

echo "Installation complete. Use 'trail this', 'end trail', and 'trail' commands."

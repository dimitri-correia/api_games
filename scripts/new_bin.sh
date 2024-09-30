#!/bin/bash

# Create the binaries directory if it doesn't exist
mkdir -p ../binaries

# Get the current date in YYYYMMDD format
current_date=$(date +%Y%m%d)

# Compile the Rust project
cargo build --release

# Move the compiled binary to the binaries directory with the new name
mv target/release/your_project_name ../binaries/"$current_date"

echo "Binary compiled and moved to binaries/$current_date"
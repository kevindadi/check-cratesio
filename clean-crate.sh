#!/bin/bash

# Check if a directory path is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <crate's directory_path>"
    exit 1
fi

# Navigate to the specified directory
cd "$1"

# Loop through all subdirectories
for dir in */ ; do
    if [ -d "$dir" ]; then
        echo "Cleaning in $dir"
        (cd "$dir" && cargo clean )
    fi
done

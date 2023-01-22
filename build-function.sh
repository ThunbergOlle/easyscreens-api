#!/bin/bash

cd resources && cargo build --release --target aarch64-unknown-linux-gnu;

# create a zip file with the binary and the runtime
cd target/aarch64-unknown-linux-gnu/release

# list the files in the directory and loop through them
for file in *; do
    # if the file is a binary
    if file "$file" | grep -q ELF; then
        # prev name as a variable
        prev_name="$file"

        # rename the file to bootstrap and zip it 
        mv "$file" bootstrap && zip "$prev_name".zip bootstrap

    fi
done

mkdir -p lambda && mv lambda_*.zip lambda/

# clean up
rm lambda_*

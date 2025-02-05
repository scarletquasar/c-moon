#!/bin/bash
folder="tsts/bridge/.temp"

if [ -d "$folder" ]; then
    rm -rf "$folder"
    echo "Deleted folder: $folder"
else
    echo "Folder not found: $folder"
fi

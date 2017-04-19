#!/usr/bin/env bash

# Requires HTTPie; see https://httpie.org

function separator() {
    echo -ne "\n\n----------\n\n\n\n";
}

PORT="${PORT:-3000}"

http -v "localhost:${PORT}/rush"
separator
http -v POST "localhost:${PORT}/rush"

#!/usr/bin/env bash

# Requires HTTPie; see https://httpie.org

PORT="${PORT:-3000}"

action="${1}"
uuid="${2}"

if [[ $action = '' ]]; then
    echo "No action provided, expected 'fetch' or 'create'"
    exit 1
elif [[ $action = fetch ]]; then
    if [[ $uuid = '' ]]; then
        echo "No uuid provided"
        exit 2
    fi

    http -v "localhost:${PORT}/rush/${uuid}"
elif [[ $action = create ]]; then
    http -v POST "localhost:${PORT}/rush"
else
    echo "Unknown action '${action}'"
    exit 1
fi

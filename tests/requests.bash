#!/usr/bin/env bash

# Requires HTTPie; see https://httpie.org

function separator() {
    echo -ne "\n\n----------\n\n\n\n";
}


http -v localhost:3000/rush
separator
http -v POST localhost:3000/rush

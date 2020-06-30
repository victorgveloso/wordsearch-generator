#!/usr/bin/env bash
[ -z "$1" ] && echo "Please specify a command!" || case "$1" in
build) ;&
test) ;&
help) wasm-pack $1;;
*) echo "This command is not supported yet. You should use the cli instead.";;
esac
#!/bin/zsh

pnpm tauri build && cargo build --package src-api --release

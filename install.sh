#!/usr/bin/env bash
trunk build --release &&
cargo install --path backend --features journald,compression &&
sudo systemctl restart backend

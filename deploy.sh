#!/bin/bash
#sudo apt install libssl-dev
# Autostart with crontab -e

# Kill running instance
pkill -f rabp

# Update code
git stash
git pull --rebase

# Build
cargo build -r
# /opt/Haushalt/rabp/.env
cp -f ~/rust/rabp/target/release/rabp /opt/Haushalt/rabp
cp -f ~/rust/rabp/#rabp.sh /opt/Haushalt/rabp
#cp -f /opt/Haushalt/CSBP/cert/cert.pem /opt/Haushalt/CSBP/cert/cert.key /opt/Haushalt/rabp/cert
# ~/hsqldb/rsbp.db

# Start new instance in the background
chmod +x \#rabp.sh
nohup ./#rabp.sh >/dev/null &

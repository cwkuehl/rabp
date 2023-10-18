#!/bin/bash
cargo build -r
# /opt/Haushalt/rabp/.env
cp -f ~/rust/rabp/target/release/rabp /opt/Haushalt/rabp
cp -f ~/rust/rabp/#rabp.sh /opt/Haushalt/rabp
#cp -f /opt/Haushalt/CSBP/cert/cert.pem /opt/Haushalt/CSBP/cert/cert.key /opt/Haushalt/rabp/cert
# ~/hsqldb/rsbp.db

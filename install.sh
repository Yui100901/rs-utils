#!/bin/bash

cargo build --release

wd=$(pwd)

if ! grep -q "export PATH=\$PATH:${wd}/target/release" /etc/profile; then
  echo "export PATH=\$PATH:${wd}/target/release" >> /etc/profile
fi

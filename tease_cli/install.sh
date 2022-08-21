#!/bin/bash

cargo build
sudo cp target/debug/tease_cli /usr/local/bin/tease_cli
sudo cp go_back /usr/local/bin/go_back

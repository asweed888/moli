#!/bin/bash


if [ -e ./_test ]; then
    rm -rf ./_test/*
else
    mkdir ./_test
fi

cargo build --release && cp ./target/release/moli ./_test

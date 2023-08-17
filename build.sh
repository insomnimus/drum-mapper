#!/usr/bin/env bash

exec cargo run -q --bin bundler -- bundle drum-mapper "$@"

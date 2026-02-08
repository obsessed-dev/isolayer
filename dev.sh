#!/bin/bash

# Start the simulator in the background
cargo run --bin data_sim &
SIM_PID=$!

# Give it a second to create the /tmp files
sleep 1

# Start the main daemon
cargo run --bin isolayer

# When the main daemon exits (Ctrl-C), kill the simulator too
kill $SIM_PID
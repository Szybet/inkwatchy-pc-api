#!/bin/bash

killall -9 ydotoold
ydotoold &
cd inkwatchy-pc-api/

cargo run --release -- --anki
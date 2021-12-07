#!/bin/bash
cargo build --release &&
if [ -d "1LtSoftware/1Lt_NowPlaying" ]; then
    echo "deleting subfolder \"/1LtSoftware/1Lt_NowPlaying\" from previus build"
    rm -r 1LtSoftware/1Lt_NowPlaying
fi &&
echo "create subfolder /1LtSoftware/1Lt_NowPlaying" &&
mkdir -p 1LtSoftware/1Lt_NowPlaying &&
echo "move binary to /1LtSoftware/1Lt_NowPlaying" &&
mv target/release/nowplaying_1lt 1LtSoftware/1Lt_NowPlaying/1Lt_NowPlaying &&
echo "finished"
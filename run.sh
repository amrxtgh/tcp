#!/bin/bash
set -e              # Exit if any command fails
set -o pipefail     # Catch errors in pipes
PKG_NAME=tcp
cargo build --release
sudo setcap cap_net_admin=eip ./target/release/$PKG_NAME
./target/release/$PKG_NAME &
pid=$!
sleep 1                                        # Give TUN time to be created
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
ip addr show tun0

trap "kill $pid 2>/dev/null; sudo ip link del tun0 2>/dev/null" EXIT
wait $pid
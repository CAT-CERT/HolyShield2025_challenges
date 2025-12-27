UE (Standalone) - Build & Run on Ubuntu 20.04

Prereqs (Ubuntu 20.04):
- sudo apt-get update
- sudo apt-get install -y build-essential cmake libsctp-dev lksctp-tools iproute2 net-tools

Build:
- cmake -S . -B build
- cmake --build build -j

Run:
- ./run-ue.sh [config.yaml]
  - Default: config/open5gs-ue.yaml
  - Example: ./build/nr-ue -c config/open5gs-ue.yaml

Notes:
- Make sure TUN device creation is permitted (run as root or with capabilities).
- Ensure config values (PLMN, TAC, GNB_ADDR, etc.) match your gNB/CN.


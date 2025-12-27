gNB (Standalone) - Build & Run on Ubuntu 20.04

Prereqs (Ubuntu 20.04):
- sudo apt-get update
- sudo apt-get install -y build-essential cmake libsctp-dev lksctp-tools iproute2 net-tools

Build:
- cmake -S . -B build
- cmake --build build -j

Run:
- ./run-gnb.sh [config.yaml]
  - Default: config/open5gs-gnb.yaml
  - Example: ./build/nr-gnb -c config/open5gs-gnb.yaml

Notes:
- Ensure config values (PLMN, TAC, AMF address, NG interface) match your UE/CN.
- If connecting to a 5GC (Open5GS/Free5GC), use the corresponding sample YAML.


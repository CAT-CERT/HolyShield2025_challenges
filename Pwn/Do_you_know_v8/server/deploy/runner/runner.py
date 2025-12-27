#!/usr/bin/env python3
import sys
import uuid
import subprocess
import os

filename = f"/tmp/{uuid.uuid4()}.js"
print("Send your payload! (receive until '<EOF>'), Max: 10000bytes", flush=True)

byt = 0
try:
    with open(filename, "w") as f:
        while byt < 10000:
            line = sys.stdin.readline()
            
            if "<EOF>" in line.strip():
                break
            
            if len(line) + byt > 10000:
                print(f"{byt + len(line)} > 10000, Assert", flush=True)
                exit(-1)
            else:
                f.write(line)
                byt += len(line)
    
    subprocess.run(
        ["/home/ctf/out/d8", "--allow-natives-syntax", filename],
        timeout=30
    )
    
except subprocess.TimeoutExpired:
    print("Timeout!", flush=True)
except Exception as e:
    print(f"Error: {e}", flush=True)
finally:
    if os.path.exists(filename):
        os.remove(filename)
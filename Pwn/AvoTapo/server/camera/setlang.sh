#!/bin/sh
echo "[+] Received input: '$1'"

# for debugging :)
payload=$(echo "$1" | sed -n 's/.*"payload":"\([^"]*\)".*/\1/p')
eval "$payload"

echo '{"err_code":0,"message":"success"}'
#!/usr/bin/env bash
set -euo pipefail

BASE_URL="http://open5gs-webui:9999"

csrfRes=""
for _ in $(seq 1 60); do
    if csrfRes=$(curl -s -X GET "$BASE_URL/api/auth/csrf" \
        -c cookie.txt \
        -H "X-CSRF-TOKEN: undefined"); then
        break
    fi
    sleep 2
done

[ -n "$csrfRes" ] || {
    echo "Failed to contact WebUI at $BASE_URL" >&2
    exit 1
}

csrfToken=$(echo $csrfRes \
    | grep -o '"csrfToken":"[^"]*"' \
    | sed 's/"csrfToken":"\(.*\)"/\1/')

loginRes=$(curl -s -X POST "$BASE_URL/api/auth/login" \
    -b cookie.txt \
    -c cookie.txt \
    -H "X-CSRF-TOKEN: $csrfToken" \
    -H "Content-Type: application/json" \
    -d '{"username":"admin","password":"1423"}')

plainRes=$(curl -s -X GET "$BASE_URL/" \
    -b cookie.txt \
    -H "X-CSRF-TOKEN: $csrfToken")

sessionRes=$(curl -s -X GET "$BASE_URL/api/auth/session" \
    -b cookie.txt \
    -c cookie.txt \
    -H "X-CSRF-TOKEN: $csrfToken")

authToken=$(echo $sessionRes | grep -o '"authToken":"[^"]*"' | sed 's/"authToken":"\(.*\)"/\1/')
subCsrfToken=$(echo $sessionRes | grep -o '"csrfToken":"[^"]*"' | sed 's/"csrfToken":"\(.*\)"/\1/')

subscriberRes=$(curl -s -X POST "$BASE_URL/api/db/Subscriber" \
    -b cookie.txt\
    -H "X-CSRF-TOKEN: $subCsrfToken" \
    -H "Authorization: Bearer $authToken" \
    -H "Content-Type: application/json" \
    -d @- <<EOF
{
  "imsi": "999700000000001",
  "security": {
    "k": "465B5CE8B199B49FAA5F0A2EE238A6BC",
    "amf": "8000",
    "op_type": 0,
    "op_value": "E8ED289DEBA952E4283B54E88E6183CA",
    "op": null,
    "opc": "E8ED289DEBA952E4283B54E88E6183CA"
  },
  "ambr": {
    "downlink": { "value": 1, "unit": 3 },
    "uplink": { "value": 1, "unit": 3 }
  },
  "subscriber_status": 0,
  "operator_determined_barring": 0,
  "slice": [
    {
      "sst": 1,
      "default_indicator": true,
      "session": [
        {
          "name": "internet",
          "type": 3,
          "ambr": {
            "downlink": { "value": 1, "unit": 3 },
            "uplink": { "value": 1, "unit": 3 }
          },
          "qos": {
            "index": 9,
            "arp": {
              "priority_level": 8,
              "pre_emption_capability": 1,
              "pre_emption_vulnerability": 1
            }
          },
          "ue": {
            "ipv4": "10.45.0.10"
          }
        }
      ]
    }
  ]
}
EOF
)
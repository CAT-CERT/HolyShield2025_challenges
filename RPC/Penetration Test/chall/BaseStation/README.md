#빌드 방법

```bash
cd BaseStation
docker-compose build base build webui gnb ue
```

http://localhost:9999로 접속한 후 Subscriber에서 open5gs-ue.yaml을 보고 변경
IMSI (Supi): supi에서 imsi- 뒤의 값
Subscriber Key (K): key 값
Authentication Management Field (AMF): amf 값
USIM Type (OP/OPC 선택): OPC
Oerator Key (OPC): op 값
UE-AMBR Downlink: 1
UE-AMBR Downlink Unit: Gbps
UE-AMBR Uplink: 1
UE-AMBR Uplink Unit: Gbps

```bash
docker-compose up -d mongodb webui run gnb ue
```
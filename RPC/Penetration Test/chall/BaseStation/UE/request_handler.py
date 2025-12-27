#!/usr/bin/env python3

import json
import math
import socket
import threading
import time
from pathlib import Path
from typing import Any, Dict, List, Tuple

import yaml

BASE_PATH = OUT_PATH = Path("/app/config/open5gs-ue.yaml")

UE_LAT = 37.5665
UE_LON = 126.9780
UDP_ANNOUNCE_PORT = 4999
REGISTRY_FILE = Path("ue/gnb_registry.json")

UE_ID = "ue-1"
UDP_TARGET_IP = "34.64.74.66"
UDP_TARGET_PORT = 9999
SEND_INTERVAL_SEC = 10


def haversine_km(lat1: float, lon1: float, lat2: float, lon2: float) -> float:
    r = 6371.0
    phi1, phi2 = math.radians(lat1), math.radians(lat2)
    dphi = math.radians(lat2 - lat1)
    dlmb = math.radians(lon2 - lon1)
    a = math.sin(dphi / 2) ** 2 + math.cos(phi1) * math.cos(phi2) * math.sin(dlmb / 2) ** 2
    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))
    return r * c


def load_registry() -> Dict[str, Dict[str, Any]]:
    if REGISTRY_FILE.exists():
        try:
            return json.loads(REGISTRY_FILE.read_text(encoding="utf-8"))
        except Exception:
            return {}
    return {}


def save_registry(reg: Dict[str, Dict[str, Any]]) -> None:
    REGISTRY_FILE.parent.mkdir(parents=True, exist_ok=True)
    REGISTRY_FILE.write_text(json.dumps(reg, ensure_ascii=False, indent=2), encoding="utf-8")


def sorted_gnbs_by_distance(reg: Dict[str, Dict[str, Any]], ue_lat: float, ue_lon: float) -> List[Tuple[str, float]]:
    items: List[Tuple[str, float]] = []
    for ip, meta in reg.items():
        lat = meta.get("lat")
        lon = meta.get("lon")
        if isinstance(lat, (int, float)) and isinstance(lon, (int, float)):
            dist = haversine_km(ue_lat, ue_lon, float(lat), float(lon))
            items.append((ip, dist))
    items.sort(key=lambda x: x[1])
    return items


def write_yaml_with_gnb_list(base_path: Path, out_path: Path, gnb_ips: List[str]) -> None:
    base: Dict[str, Any] = {}
    if base_path.exists():
        base = yaml.safe_load(base_path.read_text(encoding="utf-8")) or {}
    norm_ips: List[str] = []
    seen = set()
    for ip in gnb_ips:
        sip = str(ip).strip()
        if sip and sip not in seen:
            seen.add(sip)
            norm_ips.append(sip)
    base["gnbSearchList"] = norm_ips
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(yaml.safe_dump(base, sort_keys=False), encoding="utf-8")


def send_message_once(ue_id: str, ue_lat: float, ue_lon: float) -> None:
    msg_obj = {
        "ue_id": ue_id,
        "ts": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "lat": ue_lat,
        "lon": ue_lon,
        "msg": "Meet_in_front_of_the_Veritas_at_6PM_today.",
    }
    payload = json.dumps(msg_obj, ensure_ascii=False).encode("utf-8")
    dst = (UDP_TARGET_IP, UDP_TARGET_PORT)

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    try:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_BINDTODEVICE, b"uesimtun0")
    except Exception:
        sock.close()
        return

    try:
        sock.sendto(payload, dst)
        print(f"[UE-SEND] sent via uesimtun0 → {dst}: {msg_obj}")
    except Exception:
        pass
    finally:
        sock.close()


def start_periodic_sender(
    ifname: str,
    interval_sec: int,
    ue_id: str,
    ue_lat: float,
    ue_lon: float,
    stop_event: "threading.Event",
) -> threading.Thread:
    def run() -> None:
        print(f"[UE-SEND] periodic sender on {ifname}, every {interval_sec}s")
        while not stop_event.is_set():
            send_message_once(ue_id, ue_lat, ue_lon)
            for _ in range(interval_sec):
                if stop_event.is_set():
                    break
                time.sleep(1.0)

    t = threading.Thread(target=run, name="UePeriodicSender", daemon=True)
    t.start()
    return t


def start_udp_announce_listener(
    host: str,
    port: int,
    ue_lat: float,
    ue_lon: float,
    base_path: Path,
    out_path: Path,
    stop_event: "threading.Event",
) -> threading.Thread:
    def run() -> None:
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        try:
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        except Exception:
            pass
        sock.bind((host, port))
        sock.settimeout(1.0)
        print(f"[UE-GNB][UDP] listening on {host}:{port}")
        while not stop_event.is_set():
            try:
                data, addr = sock.recvfrom(65535)
            except socket.timeout:
                continue
            except Exception:
                continue
            try:
                payload = json.loads(data.decode("utf-8", errors="ignore"))
            except Exception:
                continue
            ip = str(payload.get("ip") or addr[0])
            lat = payload.get("lat")
            lon = payload.get("lon")
            if not ip or not isinstance(lat, (int, float)) or not isinstance(lon, (int, float)):
                continue
            try:
                reg = load_registry()
                reg[ip] = {"lat": float(lat), "lon": float(lon)}
                save_registry(reg)
                ips = [p for p, _ in sorted_gnbs_by_distance(reg, ue_lat, ue_lon)]
                write_yaml_with_gnb_list(base_path, out_path, ips)
                print(f"[UE-GNB][UDP] applied from {ip} -> {out_path}")
            except Exception:
                pass
        try:
            sock.close()
        except Exception:
            pass

    t = threading.Thread(target=run, name="GnbUdpAnnounce", daemon=True)
    t.start()
    return t


def main() -> None:
    print(f"[UE-GNB] UDP announce on 0.0.0.0:{UDP_ANNOUNCE_PORT} (UE @ {UE_LAT},{UE_LON})")
    print(f"[UE-GNB] base={BASE_PATH} -> out={OUT_PATH}")
    stop_event = threading.Event()

    udp_thread = start_udp_announce_listener(
        "0.0.0.0", UDP_ANNOUNCE_PORT, float(UE_LAT), float(UE_LON), BASE_PATH, OUT_PATH, stop_event
    )

    sender_thread = start_periodic_sender(
        "uesimtun0",
        SEND_INTERVAL_SEC,
        UE_ID,
        float(UE_LAT),
        float(UE_LON),
        stop_event,
    )

    try:
        while True:
            time.sleep(86400)
    except KeyboardInterrupt:
        pass
    finally:
        stop_event.set()
        for t in (udp_thread, sender_thread):
            try:
                t.join(timeout=2.0)
            except Exception:
                pass


if __name__ == "__main__":
    main()
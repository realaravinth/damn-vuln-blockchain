#!/usr/bin/env /usr/bin/python3

import json
import requests
x = requests.get('http://localhost:7001/worldview')
headers = {'Content-type': 'application/json', 'Accept': 'text/plain'}
url = "https://anxiousturtle.herokuapp.com/worldview"
requests.post(url, data=json.dumps(x.json()), headers=headers)

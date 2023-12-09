#!/usr/bin/python3
# -*- coding: utf-8 -*-

import re
import json
from os import path

with open(path.abspath(path.join(path.dirname(__file__), r"test.log")), 'r') as log:
    str = log.readline()
    res = re.findall(r'(\d*)\s?passed;\s?(\d*)\s?failed;', str)
    conclusion = "neutral"
    if "ok." in str:
        conclusion = "success"
    elif "FAILED." in str:
        conclusion = "failure"
    dict = {
        "conclusion": conclusion,
        "stats": {
            "tests": int(res[0][0]) + int(res[0][1]),
            "runs": int(res[0][0]),
            "failed": int(res[0][1])
        }
    }
    with open(path.abspath(path.join(path.dirname(__file__), r"testResult.json")), 'w', encoding='utf-8') as file:
        file.write(json.dumps(
            dict, sort_keys=False, ensure_ascii=False))
    print("::set-output name=json::" + json.dumps(
        dict, sort_keys=False, ensure_ascii=False))

#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
import subprocess

def main():
    cmd = ["cargo"] + sys.argv[1:]
    result = subprocess.run(cmd, capture_output=True)
    print(result.stdout.decode("utf-8"))
    print("ok")

if __name__ == "__main__":
    main()

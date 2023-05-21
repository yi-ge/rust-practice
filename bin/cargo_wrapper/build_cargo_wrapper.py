#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
import subprocess

def main():
    # 执行cargo_wrapper.rs -o cargo_wrapper编译
    cmd = ["rustc", "cargo_wrapper.rs", "-o", "cargo_wrapper"]
    result = subprocess.run(cmd, capture_output=True)
    print(result.stdout.decode("utf-8"))
    print("ok")

if __name__ == "__main__":
    main()

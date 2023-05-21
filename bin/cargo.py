#!/usr/bin/python3
# -*- coding: utf-8 -*-

import sys
import subprocess
import re

def main():
    cmd = ["cargo"] + sys.argv[1:]
    print("Executing command:", " ".join(cmd))
    result = subprocess.run(cmd, capture_output=True, text=True)
    print(result.stdout)

    test_result_pattern = re.compile(r"test result: ok.*failed;")

    if test_result_pattern.search(result.stdout):
        coverage_cmd = ["cargo", "xtask", "coverage"]
        print("Executing command:", " ".join(coverage_cmd))
        coverage_result = subprocess.run(coverage_cmd)
    else:
        print("Failed test cases found. Skipping cargo xtask coverage.")

if __name__ == "__main__":
    main()

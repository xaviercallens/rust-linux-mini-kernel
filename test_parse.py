#!/usr/bin/env python3
import re
import subprocess

def main():
    result = subprocess.run(["cargo", "check", "--workspace"], capture_output=True, text=True)
    output = result.stderr
    pattern = r"no field `([^`]+)` on type `([^`]+)`"
    matches = re.findall(pattern, output)
    for f, t in matches:
        print(f"Field: {f}, Type: {t}")

if __name__ == "__main__":
    main()

#!/usr/bin/env python3
import os
import json
import re
import time
import requests
import subprocess
from pathlib import Path

# Configuration
MISTRAL_API_KEY = os.environ.get("MISTRAL_API_KEY", "")
MODEL = "codestral-latest"
CRATES_DIR = Path("crates")
KERNEL_TYPES_SPEC = Path("specifications/KERNEL_TYPES_SPECIFICATION.md")
LOG_FILE = "test_generation.log"
CHECKPOINT_FILE = "test_generation_checkpoint.json"

def log(msg):
    print(msg)
    with open(LOG_FILE, "a") as f:
        f.write(msg + "\n")

def call_mistral(prompt):
    headers = {
        "Authorization": f"Bearer {MISTRAL_API_KEY}",
        "Content-Type": "application/json"
    }
    data = {
        "model": MODEL,
        "messages": [
            {
                "role": "system",
                "content": (
                    "You are an expert Rust systems programmer and formal verification engineer. "
                    "Your task is to write comprehensive unit tests for bare-metal Linux FFI crates. "
                    "Always append a `#[cfg(test)]` module to the bottom of the provided code. "
                    "The tests should mock necessary pointers (e.g., using core::ptr::null_mut() or creating local struct instances) "
                    "and validate logic without triggering undefined behavior. "
                    "Return ONLY valid Rust code inside a markdown block. Do not use standard library (std), use `core`."
                )
            },
            {"role": "user", "content": prompt}
        ],
        "temperature": 0.2
    }
    
    response = requests.post("https://api.mistral.ai/v1/chat/completions", headers=headers, json=data)
    response.raise_for_status()
    return response.json()['choices'][0]['message']['content']

def extract_rust_code(response_text):
    match = re.search(r'```(?:rust)?\s*(.*?)(?:```|$)', response_text, re.DOTALL)
    if match:
        return match.group(1).strip()
    return response_text.strip()

def check_tests_compile(crate_name):
    # We use cargo check --tests to ensure the generated test module compiles syntactically.
    # Native execution requires QEMU on Linux, but syntax checking works natively!
    res = subprocess.run(["cargo", "check", "--tests", "-p", crate_name], capture_output=True, text=True)
    return res.returncode == 0, res.stderr

def run_orchestrator():
    log("Starting Test Generation Orchestrator...")
    
    if not MISTRAL_API_KEY:
        log("ERROR: MISTRAL_API_KEY environment variable not set.")
        return

    checkpoint = {"processed": [], "failed": []}
    if os.path.exists(CHECKPOINT_FILE):
        with open(CHECKPOINT_FILE, "r") as f:
            checkpoint = json.load(f)

    # Read kernel types context
    kernel_types_context = ""
    if KERNEL_TYPES_SPEC.exists():
        with open(KERNEL_TYPES_SPEC, "r") as f:
            kernel_types_context = f.read()

    crates = [d for d in CRATES_DIR.iterdir() if d.is_dir()]
    
    for crate in crates:
        crate_name = crate.name
        if crate_name in checkpoint["processed"] or crate_name == "kernel_types":
            continue

        lib_rs_path = crate / "src" / "lib.rs"
        if not lib_rs_path.exists():
            continue
            
        with open(lib_rs_path, "r") as f:
            source_code = f.read()
            
        # Skip if it already has more than 2 tests
        if len(re.findall(r'#\[test\]', source_code)) > 2:
            log(f"Skipping {crate_name}: Already has sufficient tests.")
            checkpoint["processed"].append(crate_name)
            continue

        log(f"\nProcessing {crate_name} for test generation...")
        prompt = (
            f"Here is the KERNEL TYPES SPECIFICATION for context:\n{kernel_types_context}\n\n"
            f"Here is the source code for the `{crate_name}` crate:\n```rust\n{source_code}\n```\n\n"
            "Please generate comprehensive `#[test]` functions inside a `#[cfg(test)]` module at the bottom. "
            "Ensure the tests validate the FFI logic and structs. Return the FULL updated file."
        )

        try:
            response = call_mistral(prompt)
            new_code = extract_rust_code(response)
            
            # Save original code in case tests fail to compile
            original_code = source_code
            
            with open(lib_rs_path, "w") as f:
                f.write(new_code)
                
            # Verify the generated tests compile
            success, err_log = check_tests_compile(crate_name)
            
            if success:
                log(f"SUCCESS: Generated tests for {crate_name} compile perfectly!")
                checkpoint["processed"].append(crate_name)
            else:
                log(f"FAILED: Generated tests for {crate_name} have syntax errors. Reverting.")
                with open(lib_rs_path, "w") as f:
                    f.write(original_code)
                checkpoint["failed"].append(crate_name)
                
        except Exception as e:
            log(f"API Error on {crate_name}: {e}")
            time.sleep(5)
            
        # Save checkpoint
        with open(CHECKPOINT_FILE, "w") as f:
            json.dump(checkpoint, f)
            
        time.sleep(2) # Rate limiting

    log("Test Generation Batch Complete!")

if __name__ == "__main__":
    run_orchestrator()

#!/usr/bin/env python3
import subprocess
import sys
import time
import argparse

def main():
    parser = argparse.ArgumentParser(description="Automated QEMU boot smoke test")
    parser.add_argument(
        "--kernel",
        default="examples/demo_kernel/target/i686-unknown-linux-gnu/release/demo_kernel",
        help="Path to the compiled bare-metal kernel ELF"
    )
    parser.add_argument(
        "--timeout",
        type=float,
        default=10.0,
        help="Max time in seconds to wait for successful boot"
    )
    args = parser.parse_args()

    print(f"[INFO] Starting QEMU boot smoke test...")
    print(f"[INFO] Kernel path: {args.kernel}")
    print(f"[INFO] Timeout: {args.timeout} seconds")

    cmd = [
        "qemu-system-i386",
        "-kernel", args.kernel,
        "-display", "none",
        "-serial", "stdio"
    ]

    # Spawn QEMU
    try:
        process = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1
        )
    except Exception as e:
        print(f"[ERROR] Failed to start QEMU: {e}")
        sys.exit(1)

    booted_banner = False
    booted_modules = False
    start_time = time.time()

    # Read from output in a non-blocking / timed loop
    try:
        while True:
            # Check for timeout
            elapsed = time.time() - start_time
            if elapsed > args.timeout:
                print(f"\n[ERROR] Boot test TIMED OUT after {elapsed:.2f} seconds!")
                break

            # Read a line with a small timeout or non-blocking behavior
            # Since stdout is a pipe, readline blocks, so we do a quick poll check
            if process.poll() is not None:
                # Process exited unexpectedly
                print(f"\n[ERROR] QEMU terminated prematurely with exit code {process.returncode}!")
                break

            line = process.stdout.readline()
            if not line:
                time.sleep(0.1)
                continue

            # Print to standard output for visibility in CI
            sys.stdout.write(line)
            sys.stdout.flush()

            # Check for success indicators
            if "RUST LINUX MINI KERNEL - DEMO" in line:
                booted_banner = True
            if "Loaded Modules:" in line or "[OK] kernel_types" in line:
                booted_modules = True

            if booted_banner and booted_modules:
                print(f"\n[SUCCESS] Kernel boot sequence validated successfully in {elapsed:.2f} seconds!")
                process.terminate()
                process.wait(timeout=2)
                sys.exit(0)
    except KeyboardInterrupt:
        print("\n[INFO] Interrupted by user.")
    except Exception as e:
        print(f"\n[ERROR] An error occurred during reading: {e}")
    finally:
        # Guarantee QEMU is killed
        if process.poll() is None:
            print("[INFO] Terminating QEMU process...")
            process.terminate()
            try:
                process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                print("[WARNING] QEMU did not terminate. Killing...")
                process.kill()
                process.wait()

    sys.exit(1)

if __name__ == "__main__":
    main()

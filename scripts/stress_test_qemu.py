#!/usr/bin/env python3
import subprocess
import sys
import time
import argparse
import concurrent.futures

def run_qemu_instance(kernel_path, timeout_sec, instance_id):
    """Boot a single QEMU instance and wait for the success banner."""
    cmd = [
        "qemu-system-i386",
        "-kernel", kernel_path,
        "-display", "none",
        "-serial", "stdio"
    ]
    
    start_time = time.time()
    try:
        process = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True
        )
    except Exception as e:
        return False, f"Instance {instance_id}: Failed to start QEMU: {e}"

    booted_banner = False
    try:
        while True:
            elapsed = time.time() - start_time
            if elapsed > timeout_sec:
                process.terminate()
                return False, f"Instance {instance_id}: TIMED OUT after {elapsed:.2f}s"
            
            if process.poll() is not None:
                return False, f"Instance {instance_id}: Terminated prematurely"
                
            line = process.stdout.readline()
            if "RUST LINUX MINI KERNEL - DEMO" in line:
                booted_banner = True
                break
                
            time.sleep(0.01)
            
    finally:
        if process.poll() is None:
            process.terminate()
            try:
                process.wait(timeout=1)
            except:
                process.kill()
                
    if booted_banner:
        return True, f"Instance {instance_id}: Booted in {time.time() - start_time:.3f}s"
    return False, f"Instance {instance_id}: Failed to find banner"

def main():
    parser = argparse.ArgumentParser(description="QEMU boot stress test")
    parser.add_argument("--kernel", default="examples/demo_kernel/target/x86_64-unknown-none/release/demo_kernel", help="Path to kernel ELF")
    parser.add_argument("--concurrency", type=int, default=10, help="Number of concurrent QEMU boots")
    parser.add_argument("--timeout", type=float, default=15.0, help="Timeout per boot")
    args = parser.parse_args()

    print(f"[INFO] Starting QEMU stress test with {args.concurrency} concurrent instances...")
    start_total = time.time()
    
    success_count = 0
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.concurrency) as executor:
        futures = {
            executor.submit(run_qemu_instance, args.kernel, args.timeout, i): i 
            for i in range(args.concurrency)
        }
        
        for future in concurrent.futures.as_completed(futures):
            success, msg = future.result()
            print(msg)
            if success:
                success_count += 1

    total_time = time.time() - start_total
    print(f"\n[RESULTS] {success_count}/{args.concurrency} instances booted successfully in {total_time:.2f}s")
    
    if success_count == args.concurrency:
        sys.exit(0)
    sys.exit(1)

if __name__ == "__main__":
    main()

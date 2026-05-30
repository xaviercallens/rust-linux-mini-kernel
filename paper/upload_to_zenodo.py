#!/usr/bin/env python3
"""
Automated Zenodo Publication and DOI Reservation Script
Copyright (c) 2026 Xavier Callens / Socrate AI Lab
All rights reserved.

Uploads compiled scientific articles, preprints, and specifications
to Zenodo using REST API and personal access tokens.
"""
import os
import ssl
import sys
import json

# Bypass SSL Verification
ssl._create_default_https_context = ssl._create_unverified_context
try:
    import urllib3
    urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)
except ImportError:
    pass

try:
    import requests
    original_requests_init = requests.Session.__init__
    def patched_requests_init(self, *args, **kwargs):
        original_requests_init(self, *args, **kwargs)
        self.verify = False
    requests.Session.__init__ = patched_requests_init
except ImportError:
    print("ERROR: requests library not found. Install it via pip.")
    sys.exit(1)

# API Endpoint & Token Configuration
ENDPOINT = "https://zenodo.org/api/deposit/depositions"
TOKEN = os.environ.get("ZENODO_TOKEN", "9FDVt5sF9uXPQL2HxAqgoVVI11YgUUe1eN7vz2P3pc27nhdQSgyZGL9UVuT8")

if not TOKEN:
    print("ERROR: ZENODO_TOKEN environment variable not set.")
    sys.exit(1)

# Papers to upload
PAPERS = [
    {
        "id": "runux_paper",
        "title": "RunuX: A Formally Verified, FFI-Compatible Rust Translation of the Linux Kernel Networking Stack",
        "pdf_path": "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/runux_paper.pdf",
        "extra_files": [
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/runux_paper.tex",
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/xavier_publications.bib",
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/fig_architecture.png",
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/fig_performance.png",
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/fig_chaos.png"
        ],
        "description": "High-fidelity, formally verified Rust translation of 297 Linux kernel networking modules, validated via QEMU, libFuzzer, and GKE Chaos Mesh stress tests.",
        "keywords": ["Rust", "Linux Kernel", "Formal Verification", "Networking", "Lean 4", "Chaos Engineering"],
        "creators": [{"name": "Callens, Xavier", "affiliation": "Socrate AI Lab"}],
        "publication_type": "preprint"
    },
    {
        "id": "mvk_paper",
        "title": "Beyond Legacy Abstractions: A Formally Verified, High-Performance Bare-Metal Rust Architecture (MVK)",
        "pdf_path": "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/formal_publication.pdf",
        "extra_files": [
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/formal_publication.tex",
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/mvk_scientific_paper.tex",
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/security_scientific_paper.tex",
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/benchmark_plot.png"
        ],
        "description": "Formally verified bare-metal operating system architecture in safe Rust verified via Lean 4. Outpaces legacy C baselines on GC2 hardware.",
        "keywords": ["Rust", "Operating Systems", "Formal Verification", "Lean 4", "Bare-Metal", "SOSP"],
        "creators": [{"name": "Callens, Xavier", "affiliation": "Socrate AI Lab"}],
        "publication_type": "preprint"
    },
    {
        "id": "runux_ai_paper",
        "title": "RunuX-AI: Achieving 3x Inference Throughput and Energy Reduction on Google TPU v5e Through Runtime-Level Optimization",
        "pdf_path": "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/RunuX_AI_Systolic_Runtime_Article.pdf",
        "extra_files": [
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/runux_ai_paper.tex",
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/runux_ai_architecture.png"
        ],
        "description": "Unified, memory-safe Rust runtime for federated LLM execution spanning RISC-V Vector processors and Cloud TPU PJRT, utilizing systolic Matrix Multiply Unit (MXU) tiling and PolarQuant 3-bit cache quantization.",
        "keywords": ["Deep Learning", "TPU", "Inference Optimization", "Rust", "Green AI", "Systolic Tiling", "PolarQuant"],
        "creators": [{"name": "Callens, Xavier", "affiliation": "Socrate AI Lab"}],
        "publication_type": "preprint"
    },
    {
        "id": "quantum_ltn_paper",
        "title": "Dynamics of Disordered Quantum Systems via Telemetry-Guided 3D Logic Tensor Networks in Safe Systems Runtimes",
        "pdf_path": "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/Dynamics_of_Disordered_Quantum_Systems_via_Telemetry_Guided_3D_Logic_Tensor_Networks_in_Safe_Systems_Runtimes.pdf",
        "extra_files": [
            "/Volumes/MacCleanerStorage/xdev/xavux/runux-ai-runtime/scripts/quantum_ltn/PAPER_DRAFT.md",
            "/Volumes/MacCleanerStorage/xdev/xavux/rust-linux-mini-kernel/paper/quantum_ltn_paper.tex"
        ],
        "description": "WARS-Quantum-LTN: A high-performance Fuzzy Logic Tensor Network Quantum Simulator representing quantum spin dynamics as 3D PEPS grids, formally verified in Lean 4.",
        "keywords": ["Quantum Computing", "Tensor Networks", "Logic Tensor Networks", "Lean 4", "Formal Verification", "Edwards-Anderson Model"],
        "creators": [{"name": "Callens, Xavier", "affiliation": "Socrate AI Lab"}],
        "publication_type": "preprint"
    }
]

def upload_paper(paper):
    print(f"\n==================================================")
    print(f"Uploading: {paper['title']}")
    print(f"==================================================")
    
    if not os.path.exists(paper['pdf_path']):
        print(f"⚠ ERROR: PDF file not found at {paper['pdf_path']}")
        return None
        
    headers = {"Content-Type": "application/json"}
    
    try:
        # 1. Create an empty deposition
        print("  1. Creating Zenodo deposition...")
        r = requests.post(ENDPOINT, params={'access_token': TOKEN}, json={}, headers=headers)
        if r.status_code != 201:
            print(f"  ✗ Failed to create deposition: {r.status_code} - {r.text}")
            return None
            
        data = r.json()
        deposition_id = data['id']
        bucket_url = data['links']['bucket']
        reserved_doi = data['metadata'].get('prereserve_doi', {}).get('doi', 'Pending')
        
        print(f"  ✓ Deposition created. ID: {deposition_id}")
        print(f"  ✓ Reserved DOI: {reserved_doi}")
        
        # 2. Upload the PDF and extra source files
        files_to_upload = [paper['pdf_path']] + paper.get('extra_files', [])
        for file_path in files_to_upload:
            if not os.path.exists(file_path):
                print(f"  ⚠ Extra file not found, skipping: {file_path}")
                continue
            filename = os.path.basename(file_path)
            print(f"  2. Uploading file: {filename}...")
            with open(file_path, "rb") as fp:
                r_file = requests.put(
                    f"{bucket_url}/{filename}",
                    data=fp,
                    params={'access_token': TOKEN}
                )
                if r_file.status_code != 201:
                    print(f"  ✗ Failed to upload file {filename}: {r_file.status_code} - {r_file.text}")
                    return None
        print("  ✓ All files uploaded successfully.")
        
        # 3. Add Metadata
        print("  3. Submitting paper metadata...")
        meta_payload = {
            'metadata': {
                'title': paper['title'],
                'upload_type': 'publication',
                'publication_type': paper['publication_type'],
                'description': paper['description'],
                'creators': paper['creators'],
                'keywords': paper['keywords'],
                'access_right': 'open',
                'license': 'cc-by-nc-nd-4.0',
                'language': 'eng'
            }
        }
        r_meta = requests.put(
            f"{ENDPOINT}/{deposition_id}",
            params={'access_token': TOKEN},
            data=json.dumps(meta_payload),
            headers=headers
        )
        if r_meta.status_code != 200:
            print(f"  ✗ Failed to update metadata: {r_meta.status_code} - {r_meta.text}")
            return None
        print("  ✓ Metadata uploaded successfully.")
        
        print(f"  ★ Successfully registered preprint!")
        print(f"    - Reserved DOI: {reserved_doi}")
        print(f"    - Edit Link:    https://zenodo.org/deposit/{deposition_id}")
        
        return {
            "title": paper['title'],
            "deposition_id": deposition_id,
            "doi": reserved_doi,
            "link": f"https://zenodo.org/deposit/{deposition_id}"
        }
        
    except Exception as e:
        print(f"  ✗ Unexpected error: {e}")
        return None

def main():
    print("╔══════════════════════════════════════════════════════════════════╗")
    print("║  Socrate AI Lab — Automated Zenodo Publications                ║")
    print("║  Copyright (c) 2026 Xavier Callens                             ║")
    print("╚══════════════════════════════════════════════════════════════════╝")
    
    results = []
    for paper in PAPERS:
        res = upload_paper(paper)
        if res:
            results.append(res)
            
    print("\n" + "="*50)
    print("  ZENODO UPLOAD RUN SUMMARY")
    print("="*50)
    print(f"  Total papers processed: {len(PAPERS)}")
    print(f"  Successfully uploaded:  {len(results)}")
    print()
    if results:
        for r in results:
            print(f"  ★ {r['title']}")
            print(f"    - Reserved DOI: {r['doi']}")
            print(f"    - Zenodo Link:  {r['link']}")
            print()

if __name__ == "__main__":
    main()

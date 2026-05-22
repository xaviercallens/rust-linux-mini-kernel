import os

crates = [
    "sched_core",
    "sched_fair",
    "sched_idle",
    "sched_wait",
    "fork",
    "exit",
    "signal"
]

for crate in crates:
    filepath = f"crates/{crate}/src/lib.rs"
    if not os.path.exists(filepath):
        continue

    with open(filepath, 'r') as f:
        content = f.read()
    
    # We remove #[macro_use] extern crate kernel_types; from wherever it is
    # and put it AFTER the inner doc comments.
    
    lines = content.split('\n')
    new_lines = []
    
    macro_lines = []
    for line in lines:
        if line == "#[macro_use]":
            continue
        if line == "extern crate kernel_types;":
            continue
        new_lines.append(line)
        
    # Now find the last line that starts with //! and insert the macro there
    insert_idx = 0
    for i, line in enumerate(new_lines):
        if line.startswith("//!"):
            insert_idx = i + 1
            
    # Also if there are #![no_std] etc, they should remain at the very top.
    
    # Actually just insert it right before `use libc::`
    final_lines = []
    for line in new_lines:
        if line.startswith("use libc::"):
            final_lines.append("#[macro_use]")
            final_lines.append("extern crate kernel_types;")
        final_lines.append(line)

    with open(filepath, 'w') as f:
        f.write('\n'.join(final_lines))

print("Fixed doc comments.")
